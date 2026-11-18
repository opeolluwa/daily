import type { CreateUserProfile, UpdateUserProfile, UserProfile } from "@daily/core";
import { BaseRepository } from "../base";

export type { CreateUserProfile, UpdateUserProfile };

export class UserProfileRepository extends BaseRepository {
  async get_or_create(): Promise<UserProfile> {
    const existing = await this.row<UserProfile>(
      `SELECT * FROM user_profile WHERE 1=1 LIMIT 1`,
    );
    if (existing) return existing;

    const now = this.now();
    return this.mustRow<UserProfile>(
      `INSERT INTO user_profile
         (identifier, first_name, last_name, email, profile_picture, created_at, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7)
       RETURNING *`,
      [this.newId(), "", "", "", null, now, now],
    );
  }

  async update(identifier: string, payload: UpdateUserProfile): Promise<UserProfile> {
    const sets: string[] = ["updated_at = $2"];
    const params: unknown[] = [identifier, this.now()];
    let idx = 3;

    if (payload.firstName !== undefined) {
      sets.push(`first_name = $${idx++}`);
      params.push(payload.firstName);
    }
    if (payload.lastName !== undefined) {
      sets.push(`last_name = $${idx++}`);
      params.push(payload.lastName);
    }
    if (payload.email !== undefined) {
      sets.push(`email = $${idx++}`);
      params.push(payload.email);
    }
    if (payload.profilePicture !== undefined) {
      sets.push(`profile_picture = $${idx++}`);
      params.push(payload.profilePicture);
    }

    const row = await this.row<UserProfile>(
      `UPDATE user_profile SET ${sets.join(", ")} WHERE identifier = $1
       RETURNING *`,
      params,
    );
    if (!row) throw new Error("user profile not found");
    return row;
  }
}

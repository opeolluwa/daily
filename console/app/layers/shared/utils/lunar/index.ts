import type { PGlite } from "@electric-sql/pglite";
import { lunarDb } from "./pglite";
import { NotesRepository } from "./repositories/notes";
import { TaskRepository } from "./repositories/tasks";
import { BookmarkRepository } from "./repositories/bookmarks";
import { SnippetRepository } from "./repositories/snippets";
import { PasteBinRepository } from "./repositories/paste_bins";
import { ReminderRepository } from "./repositories/reminder";
import { NotificationRepository } from "./repositories/notifications";
import { RecycleBinRepository } from "./repositories/recycle_bin";
import { WorkspaceRepository } from "./repositories/workspace";
import { WorkspaceProfileRepository } from "./repositories/workspace_profiles";
import { UserPreferencesRepository } from "./repositories/user_preferences";

export interface LunarConsoleApi {
  db: PGlite;
  notes: NotesRepository;
  tasks: TaskRepository;
  bookmarks: BookmarkRepository;
  snippets: SnippetRepository;
  pasteBins: PasteBinRepository;
  reminders: ReminderRepository;
  notifications: NotificationRepository;
  recycleBin: RecycleBinRepository;
  workspaces: WorkspaceRepository;
  workspaceProfiles: WorkspaceProfileRepository;
  userPreferences: UserPreferencesRepository;
}

/** Builds the in-browser lunar API, applying pending migrations on first use. */
export async function createLunarConsoleApi(): Promise<LunarConsoleApi> {
  const db = await lunarDb();
  return {
    db,
    notes: new NotesRepository(),
    tasks: new TaskRepository(),
    bookmarks: new BookmarkRepository(),
    snippets: new SnippetRepository(),
    pasteBins: new PasteBinRepository(),
    reminders: new ReminderRepository(),
    notifications: new NotificationRepository(),
    recycleBin: new RecycleBinRepository(),
    workspaces: new WorkspaceRepository(),
    workspaceProfiles: new WorkspaceProfileRepository(),
    userPreferences: new UserPreferencesRepository(),
  };
}

export type { RequestMeta } from "./base";
export type { CreateNote, UpdateNote } from "./repositories/notes";
export type { CreateTask, UpdateTask } from "./repositories/tasks";
export type { CreateBookmark, UpdateBookmark } from "./repositories/bookmarks";
export type { CreateSnippet, UpdateSnippet } from "./repositories/snippets";
export type { CreatePasteBin, UpdatePasteBin } from "./repositories/paste_bins";
export type { CreateReminder, UpdateReminder } from "./repositories/reminder";
export type { CreateNotification } from "./repositories/notifications";
export type { CreateRecycleBinEntry } from "./repositories/recycle_bin";
export type {
  CreateWorkspace,
  UpdateWorkspace,
} from "./repositories/workspace";
export type {
  CreateWorkspaceProfile,
  UpdateWorkspaceProfile,
} from "./repositories/workspace_profiles";
export type {
  CreateUserPreferences,
  UpdateUserPreferences,
} from "./repositories/user_preferences";

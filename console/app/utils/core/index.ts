import type { PGlite } from "@electric-sql/pglite";
import { coreDb } from "./pglite";
import { NotesRepository } from "./repositories/notes";
import { TaskRepository } from "./repositories/tasks";
import { BookmarkRepository } from "./repositories/bookmarks";
import { SnippetRepository } from "./repositories/snippets";
import { PasteBinRepository } from "./repositories/paste_bins";
import { ReminderRepository } from "./repositories/reminder";
import { BackupSettingsRepository } from "./repositories/backup_settings";
import { NotificationSettingsRepository } from "./repositories/notification_settings";
import { NotificationRepository } from "./repositories/notifications";
import { RecycleBinRepository } from "./repositories/recycle_bin";
import { WorkspaceRepository } from "./repositories/workspace";
import { UserProfileRepository } from "./repositories/user_profile";
import { WhiteboardRepository } from "./repositories/whiteboards";
import { WhiteboardFileRepository } from "./repositories/whiteboard_files";
import { SyncRepository } from "./repositories/sync";
import { TaskCategoryRepository } from "./repositories/task-categories";

export interface CoreConsoleApi {
  db: PGlite;
  notes: NotesRepository;
  tasks: TaskRepository;
  taskCategories: TaskCategoryRepository;
  bookmarks: BookmarkRepository;
  snippets: SnippetRepository;
  pasteBins: PasteBinRepository;
  reminders: ReminderRepository;
  backupSettings: BackupSettingsRepository;
  notificationSettings: NotificationSettingsRepository;
  notifications: NotificationRepository;
  recycleBin: RecycleBinRepository;
  workspaces: WorkspaceRepository;
  userProfile: UserProfileRepository;
  whiteboards: WhiteboardRepository;
  whiteboardFiles: WhiteboardFileRepository;
  sync: SyncRepository;
}

/** Builds the in-browser core API, applying pending migrations on first use. */
export async function createCoreConsoleApi(): Promise<CoreConsoleApi> {
  const db = await coreDb();
  return {
    db,
    notes: new NotesRepository(),
    tasks: new TaskRepository(),
    taskCategories: new TaskCategoryRepository(),
    bookmarks: new BookmarkRepository(),
    snippets: new SnippetRepository(),
    pasteBins: new PasteBinRepository(),
    reminders: new ReminderRepository(),
    backupSettings: new BackupSettingsRepository(),
    notificationSettings: new NotificationSettingsRepository(),
    notifications: new NotificationRepository(),
    recycleBin: new RecycleBinRepository(),
    workspaces: new WorkspaceRepository(),
    userProfile: new UserProfileRepository(),
    whiteboards: new WhiteboardRepository(),
    whiteboardFiles: new WhiteboardFileRepository(),
    sync: new SyncRepository(),
  };
}

export type { RequestMeta } from "./base";
export type { CreateNote, UpdateNote } from "./repositories/notes";
export type { CreateTask, UpdateTask } from "./repositories/tasks";
export type { CreateBookmark, UpdateBookmark } from "./repositories/bookmarks";
export type { CreateSnippet, UpdateSnippet } from "./repositories/snippets";
export type { CreatePasteBin, UpdatePasteBin } from "./repositories/paste_bins";
export type { CreateReminder, UpdateReminder } from "./repositories/reminder";
export type {
  UpdateBackupSettings,
  BackupSettings,
} from "./repositories/backup_settings";
export type {
  UpdateNotificationSettings,
  NotificationSettings,
} from "./repositories/notification_settings";
export type { CreateNotification } from "./repositories/notifications";
export type { CreateRecycleBinEntry } from "./repositories/recycle_bin";
export type {
  CreateWorkspace,
  UpdateWorkspace,
} from "./repositories/workspace";
export type {
  CreateUserProfile,
  UpdateUserProfile,
} from "./repositories/user_profile";
export type {
  CreateWhiteboard,
  UpdateWhiteboard,
} from "./repositories/whiteboards";
export type { WhiteboardFiles } from "./repositories/whiteboard_files";
export type {
  CreateTaskCategoryPayload,
  UpdateTaskCategoryPayload,
} from "./repositories/task-categories";
export type {
  SyncQueueItem,
  SyncState,
  RemoteSyncChange,
  ApplyChangesResult,
} from "./repositories/sync";

-- ============================================
-- m20261116_300000_fix_recycle_bin_todo_item_type
-- ============================================
-- Fix stale item_type values left by the todo -> task rename.

UPDATE recycle_bin SET item_type = 'task' WHERE item_type = 'todo';

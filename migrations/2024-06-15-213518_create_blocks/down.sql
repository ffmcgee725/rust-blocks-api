-- This file should undo anything in `up.sql`
-- Drop the trigger
DROP TRIGGER IF EXISTS update_blocks_updated_at ON blocks;

-- Drop the trigger function
DROP FUNCTION IF EXISTS update_updated_at_column;

-- Drop the unique indexes
DROP INDEX IF EXISTS idx_network_block;
DROP INDEX IF EXISTS idx_network_timestamp;

-- Drop the table
DROP TABLE IF EXISTS blocks;
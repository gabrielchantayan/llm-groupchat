-- Create table for setting up LLM chat history
CREATE TABLE llm_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    model TEXT NOT NULL,
    message TEXT NOT NULL,
    timestamp TEXT DEFAULT CURRENT_TIMESTAMP,
    groupchat TEXT NOT NULL
)

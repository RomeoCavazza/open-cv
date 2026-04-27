db.channel_messages.createIndex(
  { "channel_id": 1, "created_at": -1 },
  { name: "idx_channel_messages_channel_created" }
);

// Optimisation queries "non supprimés" + pagination
db.channel_messages.createIndex(
  { "channel_id": 1, "deleted_at": 1, "created_at": -1 },
  { name: "idx_channel_messages_channel_deleted_created" }
);

db.channel_messages.createIndex(
  { "message_id": 1 },
  { name: "idx_channel_messages_message_id", unique: true }
);

db.channel_messages.createIndex(
  { "server_id": 1 },
  { name: "idx_channel_messages_server" }
);

db.channel_messages.createIndex(
  { "author_id": 1 },
  { name: "idx_channel_messages_author" }
);


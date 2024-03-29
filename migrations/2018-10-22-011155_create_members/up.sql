CREATE TABLE members (
  user_id UUID NOT NULL,
  project_id UUID NOT NULL,
  permission VARCHAR NOT NULL,
  PRIMARY KEY (user_id, project_id),
  FOREIGN KEY(user_id) REFERENCES users(id),
  FOREIGN KEY(project_id) REFERENCES projects(id)
)

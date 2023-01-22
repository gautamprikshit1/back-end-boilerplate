create table questions (
    id varchar primary key,
    question varchar not null
);

CREATE TABLE options (
  id varchar primary key not null,
  text varchar,
  question_id varchar not null references questions(id),
  votes integer
);

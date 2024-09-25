-- Create the 'vector' extension within the database that is set in the docker-compose.yml
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE embeddings (model_id bigint, item_id bigint, embedding vector, PRIMARY KEY (model_id, item_id));


CREATE INDEX ON embeddings USING hnsw ((embedding::vector(3)) vector_l2_ops) WHERE (model_id = 123);

SELECT * FROM embeddings WHERE model_id = 123 ORDER BY embedding::vector(3) <=> '[3,1,2]' LIMIT 5;
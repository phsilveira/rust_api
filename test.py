import psycopg2
import numpy as np
# from psycopg2 import sql
# from pgvector import pgvector, DataType

# Establish a connection to your PostgreSQL database
conn = psycopg2.connect(
    dbname="postgres",
    user="postgres",
    password="postgres",
    host="localhost",
    port="5432"
)
cur = conn.cursor()

# # Create a table to store embeddings
# create_table_query = sql.SQL(
#     "CREATE TABLE IF NOT EXISTS embeddings (id SERIAL PRIMARY KEY, vector {});"
# ).format(pgvector(DataType.Float, 3))  # Change the data type and dimension according to your embeddings
# cur.execute(create_table_query)
# conn.commit()

# # Insert embeddings into the table
# insert_query = sql.SQL(
#     "INSERT INTO embeddings (vector) VALUES ({})"
# ).format(pgvector(DataType.Float, 3))  # Change the data type and dimension accordingly

# # Sample embeddings to insert
# embeddings_to_insert = [
#     [0.1, 0.2, 0.3],
#     [0.4, 0.5, 0.6],
#     [0.7, 0.8, 0.9]
# ]

# for emb in embeddings_to_insert:
#     cur.execute(insert_query, (emb,))
# conn.commit()

# # Fetch embeddings from the table
# select_query = "SELECT * FROM embeddings;"
# cur.execute(select_query)
# rows = cur.fetchall()

# for row in rows:
#     print("ID:", row[0])
#     print("Embedding:", row[1])

# # Now let's say you have an array and you want to find the 3 most similar embeddings
# input_array = [0.3, 0.4, 0.5]  # Change this array according to your input

# # Convert input array to pgvector format
# input_vector = pgvector(DataType.Float, 3).from_data(input_array)

# # Query to find the 3 most similar embeddings
# similar_query = sql.SQL(
#     "SELECT * FROM embeddings ORDER BY vector <-> {} LIMIT 3;"
# ).format(input_vector)

# cur.execute(similar_query)
# similar_rows = cur.fetchall()

# print("\nMost similar embeddings:")
# for similar_row in similar_rows:
#     print("ID:", similar_row[0])
#     print("Embedding:", similar_row[1])



# Enable the extension

# cur = conn.cursor()
cur.execute('CREATE EXTENSION IF NOT EXISTS vector')
# Register the vector type with your connection or cursor

from pgvector.psycopg2 import register_vector

register_vector(conn)
# Create a table

cur.execute('CREATE TABLE items2 (id bigserial PRIMARY KEY, embedding vector(3))')
# Insert a vector

embedding = np.array([1, 2, 3])
cur.execute('INSERT INTO items2 (embedding) VALUES (%s)', (embedding,))
# Get the nearest neighbors to a vector

cur.execute('SELECT * FROM items2 ORDER BY embedding <-> %s LIMIT 5', (embedding,))
cur.fetchall()


# Close the cursor and connection
cur.close()
conn.close()

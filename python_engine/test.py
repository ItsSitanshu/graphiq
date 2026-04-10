from graphiq import giq
import time

@giq.trace
def preprocess_data(filename: str):
  print(f"Processing {filename}...")
  time.sleep(0.5) # Simulate work
  return "clean_dataframe"

@giq.trace
def train_model(data, learning_rate=0.01):
  print(f"Training on {data} with lr={learning_rate}")
  time.sleep(learning_rate*1000) 
  print(f"Training on complete")
  return "model_v1"

if __name__ == "__main__":
  df = preprocess_data("users_v2.csv")
  train_model(df, learning_rate=0.05)
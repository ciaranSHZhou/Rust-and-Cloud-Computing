import boto3
import pandas as pd
from sklearn.model_selection import train_test_split

# Set up AWS credentials
s3 = boto3.resource('s3',
                    aws_access_key_id='AKIAR4HECS5HHAUT3YWT',
                    aws_secret_access_key='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')

# Set the S3 bucket and file paths
bucket_name = 'sagemaker-studio-129327339342-788031v5jv'
file_name = 'diabetes.csv'
s3_path = f's3://{bucket_name}/{file_name}'

# Read the CSV file from S3 into a Pandas DataFrame
df = pd.read_csv(s3_path)

# Split the data into training, testing, and validation sets
train_data, test_val_data = train_test_split(df, test_size=0.3, random_state=42)
val_data, test_data = train_test_split(test_val_data, test_size=0.33, random_state=42)

# Upload the split datasets back to S3
s3_train_path = f's3://{bucket_name}/train.csv'
s3_test_path = f's3://{bucket_name}/test.csv'
s3_val_path = f's3://{bucket_name}/validation.csv'

train_data.to_csv(s3_train_path, index=False)
test_data.to_csv(s3_test_path, index=False)
val_data.to_csv(s3_val_path, index=False)

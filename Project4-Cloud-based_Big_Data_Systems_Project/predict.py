import joblib

def lambda_handler(event, context):
    # Load the model from disk
    model = joblib.load('/var/task/model.pkl')
    
    # Run the model on the input data
    data = event['data'] # or however you want to retrieve the input data
    output = model.predict(data)
    
    # Return the output
    return {
        'statusCode': 200,
        'body': output.tolist() # or however you want to format the output
    }

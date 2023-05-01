# Serverless Pipline: Sentiment Analyzer 
## Description
In this project, I utilized VADER sentiment analyzer to write a program that take the payload and analyze the sentiment. The function is deployed  to AWS Lambda.

## Developer Guide
### Setup
1. create a python virtual environment. Then, activate by:  
`~/.venv/bin/activate`  
2. Install cargo lambda:
`pip3 install cargo-lambda`
3. Create a new lambda function:
`cargo lambda new YOUR_FUNCTION_NAME`
4. Build release:
`cargo lambda build --release --arm64`
5. Deploy it to AWS Lambda
`cargo lambda deploy`  

### Tools
* 'make format' to format code
* 'make lint' to lint
* 'make invoke' to test the AWS Lambda function which is  
	cargo lambda invoke --remote \
  		--data-ascii '{"sentence": "I love cats!"}' \
  		--output-format json \
  		sentiment
  		
## Language & Tool
Rust
AWS Lambda
vader_sentiment
## References

* [Rust with AWS Lambda](https://www.youtube.com/watch?v=jUTiHUTfGYo)
* [AWS Lambda Rust Runtime](https://github.com/awslabs/aws-lambda-rust-runtime)

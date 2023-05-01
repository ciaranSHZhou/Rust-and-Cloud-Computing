# Serverless Pipline: Sentiment Analyzer 
## Description
In this project, I utilized VADER sentiment analyzer to write a program that take the payload and analyze the sentiment. The function is deployed  to AWS Lambda.

## Developer Guide
### Steps to run
* 'make format' to format code
* 'make lint' to lint
* 'make release arm' which is 'cargo lambda build --release --arm64'
* 'make deploy' which is 'cargo lambda deploy'
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

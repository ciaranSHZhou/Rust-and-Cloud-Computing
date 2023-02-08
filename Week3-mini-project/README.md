# Rust with AWS Lambda: Quote of the Day
## Description
In this project, I exprimented with Rust and AWS Lambda.  
I wrote a function that return a quote based on the theme inputed from user, and deployed it to AWS Lambda

## Developer Guide
### Steps to run
* 'make format' to format code
* 'make lint' to lint
* 'make release arm' which is 'cargo lambda build --release --arm64'
* 'make deploy' which is 'cargo lambda deploy'
* 'make invoke' to test the AWS Lambda function which is  
	'cargo lambda invoke --remote \
  		--data-ascii '{"theme": "sad"}' \
  		--output-format json \
  		quote-of-the-day'
  		
## Language & Tool
Rust
AWS Lambda
## References

* [Rust with AWS Lambda](https://www.youtube.com/watch?v=jUTiHUTfGYo)

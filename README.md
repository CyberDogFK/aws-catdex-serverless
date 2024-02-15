# aws-catdex-serverless

sam deploy (for the first time with --staged, almost everywhere don't choolse anyhing, for default option)

It will be shown in aws console of the region where it is created

sam delete

aws lambda invoke \
--cli-binary-format raw-in-base64-out \
--payload '{"requestContext": { "http": {"method": "POST"} } }' \
--function-name [past_function arn]\ 
--region us-east-1 \
output.json

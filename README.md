# aws-catdex-serverless

if we build with
cargo lambda build --release --arm64. Without archiving to zip
we still can deploy it, and template.yaml will look as ususal

sam deploy (for the first time with --staged, almost everywhere don't choolse anyhing, for default option)


It will be shown in aws console of the region where it is created

sam delete

aws lambda invoke \
--cli-binary-format raw-in-base64-out \
--payload '{"requestContext": { "http": {"method": "POST"} } }' \
--function-name [past_function arn]\ 
--region us-east-1 \
output.json

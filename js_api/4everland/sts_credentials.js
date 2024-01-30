import {STSClient, AssumeRoleCommand } from '@aws-sdk/client-sts';

export async function getSTSCredentials(accessKeyId, secretAccessKey) {
    const stsClient = new STSClient({
        endpoint: 'https://endpoint.4everland.co',
        region: 'us-west-1',
        credentials: {
            accessKeyId: accessKeyId,
            secretAccessKey: secretAccessKey,
        }
    });

    const params = {
        RoleSessionName: "only-put-object",
        DurationSeconds: 3600,
        Policy: `{
            "Version": "2012-10-17",
            "Statement": [
                {
                    "Effect": "Allow",
                    "Action": [
                        "s3:PutObject",
                        "s3:AbortMultipartUpload"
                    ],
                    "Resource": [
                        "arn:aws:s3:::shivarthu-upload/website_tests/*"
                    ]
                }
            ]
        }`
    };

    try {
        const data = await stsClient.send(new AssumeRoleCommand(params));
        return data.Credentials;
    } catch (error) {
        console.error("Error fetching STS credentials:", error);
        throw error;
    }
}

// // Example usage:
// const accessKeyId = 'Your api key.';
// const secretAccessKey = 'Your api secret.';

// getSTSCredentials(accessKeyId, secretAccessKey)
//     .then(credentials => {
//         console.log(credentials);
//     })
//     .catch(error => {
//         console.error("Error:", error);
//     });

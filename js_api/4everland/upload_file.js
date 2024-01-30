import { S3 } from "@aws-sdk/client-s3";
import { Upload } from "@aws-sdk/lib-storage";

// Function to upload a file using temporary credentials

// endpoint, bucket, folder, data, file_name, key, secret, session_token
export async function uploadFile4everland(endpoint, bucket, folder, data, file_name, accessKeyId, secretAccessKey, sessionToken) {
    console.log("test",endpoint, bucket, folder, data, file_name, accessKeyId, secretAccessKey, sessionToken );
    const s3 = new S3({
        endpoint: endpoint,
        signatureVersion: 'v4',
        credentials: {
            accessKeyId: accessKeyId,
            secretAccessKey: secretAccessKey,
            sessionToken: sessionToken,
        },
        region: 'us-west-2',
    });

    const bodyData = new TextEncoder().encode(data);

    const params = {
        Bucket: bucket, // Bucket name
        Key: folder + '/' + file_name, // folder name + file name
        Body: bodyData, // Inside text
        ContentType: 'text/plain',
    };

    console.log(params);

    try {
        const task = new Upload({
          client: s3,
          queueSize: 3, // 3 MiB
          params,
        });
    
        await task.done();

      } catch (error) {
        console.error('Error uploading file:', error.message);
      }
}
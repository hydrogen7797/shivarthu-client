import { BucketClient, PinningClient } from '@4everland/upload-pin';

export async function uploadPinBlob4everland(
  bucketEndpoint,
  pinningUrl,
  bucket,
  folder,
  blob,
  fileName,
  fileType,
  accessKeyId,
  secretAccessKey,
  sessionToken,
  pinSecret
) {
  // Create Bucket Client
  const bucketClient = new BucketClient({
    accessKeyId,
    secretAccessKey,
    sessionToken,
    endpoint: bucketEndpoint
  });

  // Create Pinning Client
  const pinningClient = new PinningClient({
    baseURL: pinningUrl,
    accessToken: pinSecret
  });

  // Upload File
  const task = bucketClient.uploadObject({
    Bucket: bucket,
    Key: folder + '/' + fileName,
    Body: blob,
    ContentType: fileType
  });

  const { cid } = await task.done();

  // Pin CID
  const { requestid } = await pinningClient.addPin({
    cid
  });

  return Promise.resolve(cid);
}

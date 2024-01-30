import { BucketClient, PinningClient } from '@4everland/upload-pin'

export async function uploadPinString4everland(bucketEndpoint, pinningUrl, bucket, folder, data, fileName, fileType, accessKeyId, secretAccessKey, sessionToken, pinSecret) {

    // Create Bucket Client
    const bucketClient = new BucketClient({
        accessKeyId,
        secretAccessKey,
        sessionToken,
        endpoint: bucketEndpoint
    })


    // Create Pinning Client
    const pinningClient = new PinningClient({
        baseURL: pinningUrl,
        accessToken: pinSecret
    })

    const bodyData = new TextEncoder().encode(data);

    // Upload File
    const task = bucketClient.uploadObject({
        Bucket: bucket,
        Key: folder + '/' + fileName,
        Body: bodyData,
        ContentType: fileType,
    })


    const { cid } = await task.done()

    // pin cid
    const { requestid } = await pinningClient.addPin({
        cid
    })

    return Promise.resolve(cid);
    // specified pin
    // const result = await pinningClient.getPin(requestid)
}
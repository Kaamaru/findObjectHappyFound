import { S3Client } from '@aws-sdk/client-s3';
import { s3 } from "../config.json";
import "dotenv/config"

const s3Client = new S3Client({
    forcePathStyle: true,
    region: s3.region,
    endpoint: String(process.env.SUPABASE_S3_URL),
    credentials: {
        accessKeyId: String(process.env.SUPABAE_S3_PRIVATE_KEY),
        secretAccessKey: String(process.env.SUPABASE_PUBLIC_KEY),
    }
})

export default s3Client;
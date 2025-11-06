import type { ElysiaApp } from "../../index";
import supabase from "../../lib/supabase";
import { bucket } from "../../config.json";
import { t } from "elysia";

// Body validation
// Content type : multipart/form-data
const uploadSchema = t.Object({
    file: t.File({
        type: 'image/*'
    })
})

const url = "https://actqqjzeowuhnstovjum.supabase.co/storage/v1/object/public"

// Upload file
async function uploadFile(file: any, name: string) {
    const { data, error } = await supabase.storage.from(bucket.picture).upload(`/lostitem/${name}`, file);
    if (error) {
        return false
    } else {
        return data
    }
}

export default (app: ElysiaApp) => app
    .post("/lostitem", async ({ body, set }: { body: typeof uploadSchema.static, set: { status: number } }) => {
        const { file } = body;
        const res = await uploadFile(file, file.name);
        if (res) {
            set.status = 200;
            return { url: `${url}/${res.fullPath}` };
        }
        set.status = 400
        return { error: true };
    }, {
        body: uploadSchema
    });
import { createClient } from '@supabase/supabase-js'
import "dotenv/config"

const supabase = createClient(String(process.env.SUPABASE_URL), String(process.env.SUPABASE_PUBLIC_KEY));

export default supabase;
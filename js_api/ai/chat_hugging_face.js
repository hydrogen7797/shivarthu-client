import { HfInference } from "@huggingface/inference";

export async function chat_text(token, prompt, model) {
    const HF_ACCESS_TOKEN = token;

    const hf = new HfInference(HF_ACCESS_TOKEN);
    let data = await hf.textGeneration({
        model: model,
        inputs: prompt,
        parameters: {
            temperature: 0.9,
            top_p: 0.95,
            repetition_penalty: 1.2,
            top_k: 50,
            truncate: 1000,
            max_new_tokens: 1024,
            return_full_text: false,
        }
    });
    return data.generated_text;
}




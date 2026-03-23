import torch
from transformers import AutoModelForCausalLM, AutoTokenizer, BitsAndBytesConfig
from peft import PeftModel
import os
from typing import Optional

class ModelLoader:
    """Hugging Face LoRA 모델 로딩 및 추론 엔진"""
    
    def __init__(self, model_id: str):
        self.model_id = model_id
        self.tokenizer = None
        self.model = None
        self.device = "cuda" if torch.cuda.is_available() else "cpu"

    def load_model(self):
        print(f"Loading tokenizer for {self.model_id}...")
        self.tokenizer = AutoTokenizer.from_pretrained(self.model_id)
        
        # 4-bit 양자화 설정 (GPU가 있는 경우에만 권장)
        bnb_config = None
        if self.device == "cuda":
            print("Configuring 4-bit quantization for GPU inference...")
            bnb_config = BitsAndBytesConfig(
                load_in_4bit=True,
                bnb_4bit_compute_dtype=torch.float16,
                bnb_4bit_quant_type="nf4",
                bnb_4bit_use_double_quant=True,
            )
        else:
            print("CUDA not available. Falling back to CPU inference (slow for 3.8b models).")

        print(f"Loading Base model ({self.model_id})...")
        # LoRA는 일반적으로 base 모델 위에 adapter를 얹는 방식입니다.
        # HF에 올라온 링크가 adapter만 있는 것인지, merged 모델인지에 따라 달라집니다.
        # 일단 LoRA adapter로 가정하고 로드합니다.
        
        try:
            self.model = AutoModelForCausalLM.from_pretrained(
                self.model_id,
                quantization_config=bnb_config,
                device_map="auto" if self.device == "cuda" else None,
                trust_remote_code=True
            )
            print("Model loaded successfully.")
        except Exception as e:
            print(f"Error loading model: {e}")
            raise

    def generate_response(self, prompt: str, max_new_tokens: int = 256) -> str:
        """프롬프트를 입력받아 모델의 예측 응답을 생성합니다."""
        if not self.model or not self.tokenizer:
            raise ValueError("Model and tokenizer must be loaded first.")
            
        inputs = self.tokenizer(prompt, return_tensors="pt").to(self.device)
        
        with torch.no_grad():
            outputs = self.model.generate(
                **inputs,
                max_new_tokens=max_new_tokens,
                temperature=0.7,
                do_sample=True,
                pad_token_id=self.tokenizer.eos_token_id
            )
            
        decoded = self.tokenizer.decode(outputs[0], skip_special_tokens=True)
        # 입력 프롬프트 부분 제거 후 반환
        return decoded[len(prompt):].strip()

# 싱글톤 인스턴스 (메모리 절약)
_loader: Optional[ModelLoader] = None

def get_model_loader(model_id: str = "kimdonghwanAIengineer/nexus-flow-lora-3.8b") -> ModelLoader:
    global _loader
    if _loader is None:
        _loader = ModelLoader(model_id)
        _loader.load_model()
    return _loader

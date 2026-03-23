import sys
import torch
import transformers
from peft import PeftModel
import grpc
from concurrent import futures
import time

# gRPC에서 생성된 코드 임포트
import nexus_simulation_pb2
import nexus_simulation_pb2_grpc

def verify_environment():
    print(f"Python Version: {sys.version}")
    print(f"PyTorch Version: {torch.__version__}")
    print(f"Transformers Version: {transformers.__version__}")
    print(f"CUDA Available: {torch.cuda.is_available()}")
    if torch.cuda.is_available():
        print(f"GPU: {torch.cuda.get_device_name(0)}")
    
    print("Environment verification complete.")

if __name__ == "__main__":
    verify_environment()
    print("NEXUS-Flow AI Engine starting...")

# RunPod Deployment Guide for NEXUS-Flow AI Engine

RunPod에서 AI 엔진(gRPC)을 배포하기 위한 가이드입니다.

## 1. 컨테이너 이미지 준비
현재 제공된 `Dockerfile`을 컨테이너 레지스트리(Docker Hub, GHCR 등)에 빌드하여 푸시합니다.
```bash
docker build -t your-registry/nexus-flow-ai:latest -f engine/Dockerfile engine/
docker push your-registry/nexus-flow-ai:latest
```

## 2. RunPod 인스턴스 (Pod) 생성
1. RunPod Console에서 **Secure Cloud** 또는 **Community Cloud** 진입.
2. GPU 선택: 3.8B 모델 추론을 위해 최소 16GB VRAM (예: RTX 3090, 4090, A4000 등) 권장.
3. 템플릿: 등록해둔 Docker 이미지(`your-registry/nexus-flow-ai:latest`) 스펙 입력.
4. **Environment Variables**:
   - `HF_TOKEN`: (필요한 경우) Hugging Face 인증 토큰.
5. **Port Forwarding (TCP/HTTP)**:
   - Internal Port: `50051` (TCP)을 외부로 노출(Expose). 
   - Public IP 또는 RunPod에서 제공하는 External 포트 주소를 획득합니다.

## 3. Rust Backend 연동 (GRPC Setup)
1. RunPod Pod이 구동되면 `Connect` 탭에서 제공하는 `TCP` 주소를 확인합니다. 
   *(예: `100.10.10.10:30005` 또는 TCP Tunnel 주소)*
2. 로컬 PC 또는 백엔드가 호스팅된 서버의 `.env` 파일을 수정합니다.
```env
# backend/.env 파일 내부
GRPC_HOST=http://<RUNPOD_IP>
GRPC_PORT=<RUNPOD_EXTERNAL_PORT>
```

## 4. 모델 권한 및 인증(중요)
에러 원인이었던 `kimdonghwanAIengineer/nexus-flow-lora-3.8b` 모델이 **Private** 리포지토리라면 RunPod에 환경변수 `HUGGING_FACE_HUB_TOKEN`을 넣고, 내부적으로 `login()` 처리가 되거나 CLI 인증 통과를 구현해야 합니다.

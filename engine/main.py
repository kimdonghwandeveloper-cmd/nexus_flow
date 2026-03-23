import sys
import torch
import grpc
from concurrent import futures
import time
import json
import traceback

# gRPC 생성 코드
import nexus_simulation_pb2
import nexus_simulation_pb2_grpc

# 우리 모듈
from model_loader import get_model_loader
from prompt_template import PromptTemplate

class NexusSimulationService(nexus_simulation_pb2_grpc.NexusSimulationServiceServicer):
    """NEXUS-Flow gRPC 시뮬레이션 서비스 구현"""
    
    def __init__(self):
        print("Initializing AI Engine Simulation Service...")
        # 3.8b 모델을 로드합니다. (최초 실행 시 시간 소요)
        self.loader = get_model_loader()
        self.prompt_engine = PromptTemplate()

    def RunSimulation(self, request, context):
        """단일 시뮬레이션 요청 처리 (LLM 추론)"""
        print(f"Simulation requested: ID={request.request_id}")
        
        try:
            # 1. Protobuf에서 Python Dict로 변환 (토폴로지는 JSON 문자열로 저장되어 올 수도 있음)
            # 여기서는 Protobuf 필드 직접 접근
            parameter_changes = []
            for pc in request.parameter_changes:
                parameter_changes.append({
                    "node_id": pc.node_id,
                    "param_name": pc.param_name,
                    "new_value": pc.new_value
                })
            
            # 토폴로지 추출 (단순화: Protobuf 객체를 Dict로 변환)
            # 팁: google.protobuf.json_format.MessageToDict 사용 가산
            from google.protobuf.json_format import MessageToDict
            topology_dict = MessageToDict(request.topology)
            
            # 2. 프롬프트 생성
            prompt = self.prompt_engine.construct_simulation_prompt(topology_dict, parameter_changes)
            
            # 3. AI 추론 실행
            print("Running LLM inference...")
            llm_response = self.loader.generate_response(prompt)
            print(f"LLM Response: {llm_response}")
            
            # 4. 결과 파싱
            parsed_result = self.prompt_engine.parse_llm_response(llm_response)
            
            # 5. 응답 생성
            return nexus_simulation_pb2.SimulationResponse(
                request_id=request.request_id,
                success=parsed_result.get("success", True),
                overall_throughput=float(parsed_result.get("overall_throughput", 0.0)),
                overall_efficiency=float(parsed_result.get("overall_efficiency", 0.0)),
                # impact_chain: proto 규격에 맞춰 매핑 (현재는 간단히 텍스트 하나만)
                impact_chain=[nexus_simulation_pb2.ImpactChainLink(
                    node_id="AI_ENGINE",
                    impact_score=1.0,
                    description=parsed_result.get("impact_chain", "Analysis complete.")
                )],
                node_results=[] # 향후 고도화
            )
            
        except Exception as e:
            traceback.print_exc()
            return nexus_simulation_pb2.SimulationResponse(
                request_id=request.request_id,
                success=False,
                overall_throughput=0.0,
                overall_efficiency=0.0
            )

    def HealthCheck(self, request, context):
        """서버 상태 확인"""
        return nexus_simulation_pb2.HealthCheckResponse(
            status="READY",
            gpu_available=torch.cuda.is_available(),
            model_info=f"HF:{self.loader.model_id}"
        )

def serve():
    """gRPC 서버 실행"""
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=4))
    nexus_simulation_pb2_grpc.add_NexusSimulationServiceServicer_to_server(
        NexusSimulationService(), server
    )
    
    port = "50051"
    server.add_insecure_port(f"[::]:{port}")
    print(f"NEXUS-Flow AI Engine listening on port {port}")
    server.start()
    
    try:
        while True:
            time.sleep(86400)
    except KeyboardInterrupt:
        server.stop(0)

if __name__ == "__main__":
    serve()

import json
from typing import List, Dict, Any

class PromptTemplate:
    """토폴로지 데이터를 LLM 프롬프트로 변환하는 클래스"""
    
    @staticmethod
    def construct_simulation_prompt(topology: Dict[str, Any], parameter_changes: List[Dict[str, Any]]) -> str:
        """토폴로지 구조와 파라미터 변경 사항을 기반으로 프롬프트를 생성합니다."""
        
        nodes = topology.get("nodes", [])
        edges = topology.get("edges", [])
        
        prompt = "### System Instruction\n"
        prompt += "당신은 고성능 제조 공정 시뮬레이션 AI입니다. 아래 제공되는 공정 토폴로지(JSON)와 파라미터 변경 사항을 분석하여, 공정 전체의 처리량(Throughput)과 효율성(Efficiency)의 변화를 예측하고, 그 근거(Impact Chain)를 설명하십시오.\n\n"
        
        prompt += "### Process Topology\n"
        # 노드 정보 요약
        prompt += "Nodes:\n"
        for node in nodes:
            id = node.get("id")
            label = node.get("data", {}).get("label", id)
            params = node.get("data", {}).get("parameters", {})
            prompt += f"- {id} ({label}): Cycle Time={params.get('cycle_time')}s, Failure Rate={params.get('failure_rate')*100}%\n"
            
        # 에지(연결) 정보 요약
        prompt += "\nConnections:\n"
        for edge in edges:
            src = edge.get("source")
            tgt = edge.get("target")
            prompt += f"- {src} -> {tgt}\n"
            
        prompt += "\n### Changed Parameters\n"
        if not parameter_changes:
            prompt += "None (Baseline Simulation)\n"
        else:
            for change in parameter_changes:
                prompt += f"- {change['node_id']}: {change['param_name']} -> {change['new_value']}\n"
                
        prompt += "\n### Response Format (JSON)\n"
        prompt += '{\n  "overall_throughput": "number",\n  "overall_efficiency": "number",\n  "impact_chain": "description",\n  "success": true\n}\n\n'
        
        prompt += "### Response:\n"
        return prompt

    @staticmethod
    def parse_llm_response(response_text: str) -> Dict[str, Any]:
        """LLM의 텍스트 응답에서 JSON 데이터를 추출합니다."""
        try:
            # 단순화를 위해 텍스트 전처리 후 JSON 파싱 시도
            start = response_text.find("{")
            end = response_text.rfind("}") + 1
            if start != -1 and end != 0:
                json_part = response_text[start:end]
                return json.loads(json_part)
            else:
                raise ValueError("JSON not found in response")
        except Exception as e:
            print(f"Failed to parse LLM response: {e}")
            # 기본값 반환
            return {
                "overall_throughput": 0.0,
                "overall_efficiency": 0.0,
                "impact_chain": "Error: Failed to parse AI insights.",
                "success": False
            }

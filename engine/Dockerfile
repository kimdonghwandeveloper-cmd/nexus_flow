# Base Image: CUDA support for Hugging Face Transformers
FROM nvidia/cuda:12.1.0-base-ubuntu22.04

# Environment variables
ENV DEBIAN_FRONTEND=noninteractive
ENV PYTHONUNBUFFERED=1
ENV PORT=50051

# Install Python and dependencies
RUN apt-get update && apt-get install -y \
    python3.10 \
    python3-pip \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install uv (Fast Python Package Manager)
RUN curl -LsSf https://astral.sh/uv/install.sh | sh
ENV PATH="/root/.local/bin:${PATH}"

WORKDIR /app

# Copy dependency files
COPY pyproject.toml .

# Sync dependencies using uv
# We copy everything else after so that uv sync is cached
RUN uv sync

# Copy source code including proto mapping
# Assuming proto files are inside the container or generated beforehand
COPY . .

# Expose gRPC port. Note: RunPod requires exposing the TCP port for external services
EXPOSE 50051

# Run the AI gRPC Server
CMD ["uv", "run", "python", "main.py"]

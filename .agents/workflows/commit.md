---
description: How to commit changes following the project's commit conventions
---

# Commit Conventions

## 1. Atomic Commit
- 하나의 커밋에는 하나의 논리적 작업(기능 구현, 버그 수정 등)만 담는다.

## 2. Conventional Commits Prefix
- `feat:` 새로운 기능
- `fix:` 버그 수정
- `refactor:` 리팩토링
- `docs:` 문서 변경
- `chore:` 빌드, 설정 등 기타
- `test:` 테스트 추가/수정

## 3. Commit Timing
- 파일 개수가 아니라, 테스트를 통과했거나 논리적으로 매듭지어지는 순간에 커밋한다.

## 4. Granular Staging
- 한 파일 내에서도 성격이 다른 수정은 `git add -p` 등으로 쪼개서 커밋한다.

## Workflow Steps
// turbo-all

1. Identify the logical unit of work completed
2. Stage only the files related to that single logical unit: `git add <specific files>`
3. Commit with conventional prefix: `git commit -m "<type>: <description>"`
4. Repeat for each distinct logical unit

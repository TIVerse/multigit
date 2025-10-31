# MultiGit Architecture & Flow Diagrams

This document contains Mermaid.js diagrams illustrating how MultiGit works in real-life scenarios.

---

## 1. High-Level Architecture Overview

```mermaid
graph TB
    subgraph "User Layer"
        User[👤 User]
        CLI[🖥️ CLI Interface]
        Daemon[⚙️ Daemon Service]
    end
    
    subgraph "Core Engine"
        Config[📝 Config Manager]
        SyncMgr[🔄 Sync Manager]
        ConflictRes[⚔️ Conflict Resolver]
        Auth[🔐 Auth Manager]
    end
    
    subgraph "Git Layer"
        GitOps[📦 Git Operations<br/>libgit2 wrapper]
        Remote[🌐 Remote Manager]
        Branch[🌿 Branch Manager]
    end
    
    subgraph "Provider Adapters"
        GitHub[<img src='https://github.com/favicon.ico' width='16'/> GitHub Provider]
        GitLab[🦊 GitLab Provider]
        Bitbucket[🪣 Bitbucket Provider]
        Gitea[🍵 Gitea Provider]
    end
    
    subgraph "Security Layer"
        Keyring[🔑 OS Keyring]
        Encryption[🔒 Encrypted Store]
        Audit[📋 Audit Logger]
    end
    
    User -->|Commands| CLI
    User -->|Background Sync| Daemon
    
    CLI --> Config
    CLI --> SyncMgr
    CLI --> Auth
    
    Daemon -->|Periodic Sync| SyncMgr
    
    SyncMgr --> GitOps
    SyncMgr --> ConflictRes
    
    GitOps --> Remote
    GitOps --> Branch
    
    Remote --> GitHub
    Remote --> GitLab
    Remote --> Bitbucket
    Remote --> Gitea
    
    Auth --> Keyring
    Auth --> Encryption
    Auth --> Audit
    
    Config -.->|Load/Save| ConfigFile[(💾 config.toml)]
    Audit -.->|Write| AuditLog[(📄 audit.log)]
    
    style User fill:#e1f5ff
    style CLI fill:#ffe1f5
    style Daemon fill:#fff5e1
    style SyncMgr fill:#e1ffe1
    style Auth fill:#ffe1e1
```

---

## 2. Complete User Workflow - From Setup to Sync

```mermaid
sequenceDiagram
    actor User
    participant CLI as MultiGit CLI
    participant Config as Config Manager
    participant Auth as Auth Manager
    participant Keyring as OS Keyring
    participant Git as Git Operations
    participant Sync as Sync Manager
    participant GH as GitHub
    participant GL as GitLab
    
    Note over User,GL: 🚀 Initial Setup
    
    User->>CLI: multigit init
    CLI->>Config: Create .multigit/config.toml
    Config-->>User: ✅ Initialized
    
    Note over User,GL: 🔐 Adding Remotes
    
    User->>CLI: multigit remote add github user123
    CLI->>User: Prompt for GitHub token
    User->>CLI: Enter: ghp_xxxxx
    CLI->>GH: Test connection
    GH-->>CLI: ✅ Valid
    CLI->>Auth: Store credentials
    Auth->>Keyring: Save token securely
    CLI->>Config: Add remote config
    Config-->>User: ✅ GitHub added
    
    User->>CLI: multigit remote add gitlab user123
    CLI->>User: Prompt for GitLab token
    User->>CLI: Enter: glpat-xxxxx
    CLI->>GL: Test connection
    GL-->>CLI: ✅ Valid
    CLI->>Auth: Store credentials
    Auth->>Keyring: Save token securely
    CLI->>Config: Add remote config
    Config-->>User: ✅ GitLab added
    
    Note over User,GL: 📊 Check Status
    
    User->>CLI: multigit status
    CLI->>Git: Get current branch
    Git-->>CLI: main
    CLI->>Git: Check if clean
    Git-->>CLI: ✅ Clean
    CLI->>Config: Get enabled remotes
    Config-->>CLI: [github, gitlab]
    CLI-->>User: Display status
    
    Note over User,GL: 🚀 Multi-Remote Push
    
    User->>CLI: multigit push
    CLI->>Git: Check working directory
    Git-->>CLI: ✅ Clean
    CLI->>Config: Load enabled remotes
    Config-->>CLI: [github, gitlab]
    
    Note over Sync,GL: Parallel Push Operations
    
    CLI->>Sync: push_all("main", [github, gitlab])
    
    par Push to GitHub
        Sync->>Git: GitOps::open(".")
        Git-->>Sync: Repository instance
        Sync->>Auth: Get GitHub token
        Auth->>Keyring: Retrieve token
        Keyring-->>Auth: ghp_xxxxx
        Auth-->>Sync: Token
        Sync->>Git: push("github", "refs/heads/main")
        Git->>GH: Push commits (with timeout: 300s)
        GH-->>Git: ✅ Success
        Git-->>Sync: PushResult { success: true, duration: 1250ms }
    and Push to GitLab
        Sync->>Git: GitOps::open(".")
        Git-->>Sync: Repository instance
        Sync->>Auth: Get GitLab token
        Auth->>Keyring: Retrieve token
        Keyring-->>Auth: glpat-xxxxx
        Auth-->>Sync: Token
        Sync->>Git: push("gitlab", "refs/heads/main")
        Git->>GL: Push commits (with timeout: 300s)
        GL-->>Git: ✅ Success
        Git-->>Sync: PushResult { success: true, duration: 980ms }
    end
    
    Sync-->>CLI: Results: [2 successful, 0 failed]
    CLI-->>User: ✅ Pushed to 2 remotes<br/>✓ github - 1250ms<br/>✓ gitlab - 980ms
    
    Note over User,GL: 🔄 Fetch Updates
    
    User->>CLI: multigit fetch --all
    CLI->>Sync: fetch_all([github, gitlab])
    
    par Fetch from GitHub
        Sync->>Git: fetch("github")
        Git->>GH: Fetch updates (timeout: 300s)
        GH-->>Git: 3 new commits
        Git-->>Sync: FetchResult { commits: 3 }
    and Fetch from GitLab
        Sync->>Git: fetch("gitlab")
        Git->>GL: Fetch updates (timeout: 300s)
        GL-->>Git: 0 new commits
        Git-->>Sync: FetchResult { commits: 0 }
    end
    
    Sync-->>CLI: Results: [2 successful]
    CLI-->>User: ✅ Fetched from 2 remotes<br/>✓ github - 3 commits<br/>✓ gitlab - 0 commits
```

---

## 3. Daemon Background Sync Operation

```mermaid
sequenceDiagram
    actor User
    participant CLI as MultiGit CLI
    participant Daemon as Daemon Service
    participant Scheduler as Task Scheduler
    participant Process as tokio::process
    participant SyncCmd as multigit sync
    participant Sync as Sync Manager
    participant Git as Git Operations
    participant GH as GitHub
    participant GL as GitLab
    
    Note over User,GL: 🚀 Starting Daemon
    
    User->>CLI: multigit daemon start --interval 5m
    CLI->>Daemon: DaemonService::new(300s)
    Daemon->>Daemon: Write PID file
    Daemon->>Scheduler: Create scheduler (300s interval)
    Daemon-->>User: ✅ Daemon started (PID: 12345)
    
    Note over User,GL: ⏰ Periodic Sync (every 5 minutes)
    
    loop Every 5 minutes
        Scheduler->>Scheduler: Timer tick
        Scheduler->>Daemon: Execute sync task
        Daemon->>Config: Load config
        Config-->>Daemon: Enabled remotes: [github, gitlab]
        
        Note over Daemon,GL: Daemon spawns CLI process
        
        Daemon->>Process: Command::new("multigit")<br/>.args(["sync", "--no-interaction"])
        Process->>SyncCmd: Execute sync command
        
        SyncCmd->>Sync: Perform sync
        Sync->>Config: Get enabled remotes
        Config-->>Sync: [github, gitlab]
        
        Note over Sync,GL: Fetch Phase
        
        par Fetch from remotes
            Sync->>Git: fetch("github")
            Git->>GH: Fetch
            GH-->>Git: ✅ Updates
            Git-->>Sync: Success
        and
            Sync->>Git: fetch("gitlab")
            Git->>GL: Fetch
            GL-->>Git: ✅ No updates
            Git-->>Sync: Success
        end
        
        Note over Sync,GL: Push Phase
        
        par Push to remotes
            Sync->>Git: push("github", "main")
            Git->>GH: Push
            GH-->>Git: ✅ Success
            Git-->>Sync: Success
        and
            Sync->>Git: push("gitlab", "main")
            Git->>GL: Push
            GL-->>Git: ✅ Success
            Git-->>Sync: Success
        end
        
        SyncCmd-->>Process: Exit 0
        Process-->>Daemon: Output: Sync successful
        Daemon->>Daemon: Log: [Daemon] Sync completed
        
        Note over Scheduler: Wait 5 minutes...
    end
    
    Note over User,GL: 🛑 Stopping Daemon
    
    User->>CLI: multigit daemon stop
    CLI->>Daemon: Read PID file (12345)
    CLI->>Daemon: Send SIGTERM to PID 12345
    Daemon->>Scheduler: Stop scheduler
    Scheduler-->>Daemon: Graceful shutdown
    Daemon->>Daemon: Remove PID file
    CLI-->>User: ✅ Daemon stopped
```

---

## 4. Parallel Operations with Semaphore Control

```mermaid
graph TB
    subgraph "Sync Manager"
        Start[🚀 Start Sync Operation]
        Create[Create Semaphore<br/>max_parallel = 4]
        SpawnTasks[Spawn Tasks for Each Remote]
    end
    
    subgraph "Semaphore Pool (4 permits)"
        Sem[🎫 Semaphore<br/>Available Permits: 4]
    end
    
    subgraph "Concurrent Tasks"
        T1[Task 1: GitHub<br/>🔒 Acquire permit]
        T2[Task 2: GitLab<br/>🔒 Acquire permit]
        T3[Task 3: Bitbucket<br/>🔒 Acquire permit]
        T4[Task 4: Gitea<br/>🔒 Acquire permit]
        T5[Task 5: Codeberg<br/>⏳ Waiting...]
    end
    
    subgraph "Git Operations"
        G1[Push to GitHub<br/>⏱️ 2.5s]
        G2[Push to GitLab<br/>⏱️ 1.8s]
        G3[Push to Bitbucket<br/>⏱️ 3.2s]
        G4[Push to Gitea<br/>⏱️ 1.5s]
        G5[Push to Codeberg<br/>⏱️ 2.0s]
    end
    
    Start --> Create
    Create --> Sem
    Create --> SpawnTasks
    
    SpawnTasks --> T1
    SpawnTasks --> T2
    SpawnTasks --> T3
    SpawnTasks --> T4
    SpawnTasks --> T5
    
    T1 -.->|Permit 1| Sem
    T2 -.->|Permit 2| Sem
    T3 -.->|Permit 3| Sem
    T4 -.->|Permit 4| Sem
    T5 -.->|❌ No permits<br/>Must wait| Sem
    
    T1 --> G1
    T2 --> G2
    T3 --> G3
    T4 --> G4
    
    G2 -->|✅ Fastest<br/>Release permit| R1[🔓 Permit Released]
    R1 -.-> T5
    T5 --> G5
    
    G1 --> Done1[✅ Complete]
    G2 --> Done2[✅ Complete]
    G3 --> Done3[✅ Complete]
    G4 --> Done4[✅ Complete]
    G5 --> Done5[✅ Complete]
    
    Done1 --> Results
    Done2 --> Results
    Done3 --> Results
    Done4 --> Results
    Done5 --> Results
    
    Results[📊 Aggregate Results<br/>5 successful, 0 failed]
    
    style Sem fill:#ffe1e1
    style T5 fill:#fff5e1
    style Results fill:#e1ffe1
```

---

## 5. Authentication & Security Flow

```mermaid
graph TB
    subgraph "User Action"
        AddRemote[multigit remote add<br/>github username]
    end
    
    subgraph "Credential Input"
        Prompt[🔐 Password Prompt<br/>dialoguer]
        Input[User enters token:<br/>ghp_xxxxxxxxx]
    end
    
    subgraph "Validation"
        TestConn[Test Connection<br/>to GitHub API]
        ValidToken{Token Valid?}
    end
    
    subgraph "Storage Decision"
        Backend{Auth Backend?}
        Keyring[OS Keyring<br/>🔑]
        Encrypted[Encrypted File<br/>🔒 Age encryption]
        EnvVar[Environment Var<br/>🌍]
    end
    
    subgraph "OS Keyring Details"
        direction TB
        K1[macOS:<br/>Keychain Access]
        K2[Windows:<br/>Credential Manager]
        K3[Linux:<br/>Secret Service]
    end
    
    subgraph "Encrypted File Details"
        E1[Generate Key<br/>from Passphrase]
        E2[Age Encryption<br/>AES-256]
        E3[Store:<br/>~/.config/multigit/<br/>credentials.enc]
    end
    
    subgraph "Audit Trail"
        Audit[📋 Audit Logger]
        Log[Event: CredentialStore<br/>Provider: github<br/>User: username<br/>Success: true<br/>Timestamp: 2025-10-31]
    end
    
    subgraph "Config Update"
        ConfigSave[Save to<br/>.multigit/config.toml]
        RemoteEntry["[remotes.github]<br/>username = 'user'<br/>enabled = true"]
    end
    
    AddRemote --> Prompt
    Prompt --> Input
    Input --> TestConn
    TestConn --> ValidToken
    
    ValidToken -->|✅ Yes| Backend
    ValidToken -->|❌ No| Error[❌ Authentication<br/>Failed]
    
    Backend -->|Keyring| Keyring
    Backend -->|Encrypted| Encrypted
    Backend -->|Environment| EnvVar
    
    Keyring --> K1
    Keyring --> K2
    Keyring --> K3
    
    K1 --> Audit
    K2 --> Audit
    K3 --> Audit
    
    Encrypted --> E1
    E1 --> E2
    E2 --> E3
    E3 --> Audit
    
    EnvVar --> Audit
    
    Audit --> Log
    Log --> ConfigSave
    ConfigSave --> RemoteEntry
    RemoteEntry --> Success[✅ Remote Added<br/>Successfully]
    
    style Backend fill:#ffe1e1
    style Audit fill:#e1f5ff
    style Success fill:#e1ffe1
    style Error fill:#ffe1e1
```

---

## 6. Network Timeout Protection

```mermaid
sequenceDiagram
    participant User
    participant Sync as Sync Manager
    participant Git as Git Operations
    participant Remote as Remote Server
    participant Timeout as Timeout Monitor
    
    Note over User,Timeout: 🚀 Push Operation with Timeout
    
    User->>Sync: push_all("main", [github])
    Sync->>Git: GitOps::open(".") with 5min timeout
    Git->>Git: Set network_timeout = 300s
    
    Sync->>Git: push("github", "refs/heads/main")
    Git->>Timeout: Start timer (300s)
    
    Note over Git,Remote: Transfer in progress...
    
    loop Transfer Progress
        Git->>Remote: Send data chunk
        Remote-->>Git: ACK
        Git->>Timeout: Check elapsed time
        
        alt Time < 300s
            Timeout-->>Git: ✅ Continue
            Note over Git,Remote: Keep transferring...
        else Time >= 300s
            Timeout-->>Git: ⏰ TIMEOUT!
            Git->>Git: Abort transfer
            Git-->>Sync: Error: Push timed out after 300s
            Sync-->>User: ❌ github - Push timed out after 300s
        end
    end
    
    alt Transfer Completed in Time
        Remote-->>Git: ✅ Push successful
        Git->>Timeout: Stop timer
        Timeout-->>Git: ✅ 45s elapsed
        Git-->>Sync: PushResult { success: true, duration: 45s }
        Sync-->>User: ✅ github - pushed in 45s
    end
```

---

## 7. Real-World Usage Scenario

```mermaid
flowchart TD
    Start([👨‍💻 Developer starts work])
    
    Start --> Morning[🌅 Morning: Start Daemon]
    Morning --> DaemonStart["multigit daemon start --interval 15m<br/>✅ Background sync every 15 minutes"]
    
    DaemonStart --> Work[💻 Working on code...]
    
    Work --> Commit1[git commit -m 'Add feature']
    Commit1 --> AutoSync1[⏰ Daemon auto-syncs<br/>to GitHub, GitLab, Bitbucket]
    
    AutoSync1 --> Work2[💻 Continue coding...]
    Work2 --> Commit2[git commit -m 'Fix bug']
    Commit2 --> AutoSync2[⏰ Daemon auto-syncs again]
    
    AutoSync2 --> Lunch[🍽️ Lunch Break]
    Lunch --> AutoSync3[⏰ Daemon syncs while away]
    
    AutoSync3 --> Afternoon[☀️ Back to work]
    Afternoon --> Status["multigit status<br/>✅ All remotes in sync"]
    
    Status --> Emergency{🚨 Need quick sync?}
    Emergency -->|Yes| Manual["multigit sync<br/>⚡ Immediate sync to all"]
    Emergency -->|No| Continue[💻 Keep working]
    
    Manual --> Continue
    Continue --> EOD[🌙 End of day]
    
    EOD --> StopDaemon["multigit daemon stop<br/>✅ Daemon stopped"]
    
    StopDaemon --> Final[💾 All code safely backed up<br/>across 3 platforms]
    
    style Start fill:#e1f5ff
    style DaemonStart fill:#e1ffe1
    style AutoSync1 fill:#fff5e1
    style AutoSync2 fill:#fff5e1
    style AutoSync3 fill:#fff5e1
    style Manual fill:#ffe1e1
    style Final fill:#e1ffe1
```

---

## 8. Error Handling & Recovery

```mermaid
graph TB
    Start[🚀 Start Sync Operation]
    
    Start --> Check{Pre-flight Checks}
    
    Check -->|❌ Not initialized| E1[Error: Run multigit init]
    Check -->|❌ No remotes| E2[Error: Add remotes first]
    Check -->|❌ Dirty workspace| E3[Warning: Uncommitted changes]
    Check -->|✅ All good| Proceed[Proceed with sync]
    
    Proceed --> Fetch[📥 Fetch Phase]
    
    Fetch --> F1{GitHub Fetch}
    F1 -->|✅ Success| F2{GitLab Fetch}
    F1 -->|❌ Timeout| FR1[Retry with backoff]
    F1 -->|❌ Auth failed| FR2[Prompt: Update token]
    F1 -->|❌ Rate limit| FR3[Wait and retry]
    
    FR1 --> F2
    FR2 --> F2
    FR3 --> F2
    
    F2 -->|✅ Success| Push[📤 Push Phase]
    F2 -->|❌ Error| FR4[Log error, continue]
    
    FR4 --> Push
    
    Push --> P1{GitHub Push}
    P1 -->|✅ Success| P2{GitLab Push}
    P1 -->|❌ Conflict| PR1[Run conflict resolver]
    P1 -->|❌ Force needed| PR2[Prompt: Use --force?]
    P1 -->|❌ No access| PR3[Error: Check permissions]
    
    PR1 --> Resolve{Resolution Strategy}
    Resolve -->|Ours| KeepLocal[Keep local changes]
    Resolve -->|Theirs| AcceptRemote[Accept remote changes]
    Resolve -->|Manual| UserResolve[User resolves manually]
    
    KeepLocal --> Retry1[Retry push]
    AcceptRemote --> Retry1
    UserResolve --> Retry1
    
    Retry1 --> P2
    PR2 --> P2
    PR3 --> P2
    
    P2 -->|✅ Success| Summary[📊 Generate Summary]
    P2 -->|❌ Error| Summary
    
    Summary --> Report["Report Results:<br/>✅ 2 successful<br/>❌ 1 failed<br/>⏱️ 3.2s total"]
    
    Report --> Audit[📋 Write to Audit Log]
    Audit --> Done[✅ Complete]
    
    style Check fill:#e1f5ff
    style E1 fill:#ffe1e1
    style E2 fill:#ffe1e1
    style E3 fill:#fff5e1
    style Resolve fill:#ffe1ff
    style Done fill:#e1ffe1
```

---

## Legend

- 🚀 **Start/Launch** - Beginning of operation
- ✅ **Success** - Operation completed successfully
- ❌ **Error** - Operation failed
- ⏰ **Timer** - Time-based operation
- 🔐 **Security** - Authentication/encryption
- 📊 **Report** - Status/summary
- ⚡ **Fast** - Quick operation
- 🔄 **Sync** - Synchronization operation
- 📥 **Fetch** - Download operation
- 📤 **Push** - Upload operation
- 🔒 **Lock** - Resource acquired
- 🔓 **Unlock** - Resource released

---

**Generated**: 2025-10-31  
**Version**: MultiGit v1.0.0  
**Tool**: Mermaid.js

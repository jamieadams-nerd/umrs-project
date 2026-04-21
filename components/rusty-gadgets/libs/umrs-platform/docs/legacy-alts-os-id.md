  Now, OsIdentity. Looking at what the secure-replacement surface actually is.                                      
  
  Searched for 3 patterns, read 4 files, listed 1 directory (ctrl+o to expand)                                      
                                                                                                            
● Now I've got enough. Here's the picture of the OsIdentity / secure-replacement-for-uname surface.                 
                                          
  What legacy uname(2) returns vs what UMRS provides                                                                
                                                                                                                    
  uname(2) hands you six strings from the kernel with no provenance, no cross-check, no way to rate how much you    
  should trust them:                                                                                                
                                                                                                                    
  ┌────────────┬──────────────────────────┬───────────────────────────────────────────────────┬────────────────┐    
  │   Field    │       Legacy uname       │                 UMRS replacement                  │ Where it lives │ 
  ├────────────┼──────────────────────────┼───────────────────────────────────────────────────┼────────────────┤    
  │ sysname,   │ "Linux" (untyped)        │ OsFamily enum (RpmBased / DpkgBased / PacmanBased │ os_identity.rs │ 
  │ ID         │                          │  / Unknown) + Distro enum                         │                │ 
  ├────────────┼──────────────────────────┼───────────────────────────────────────────────────┼────────────────┤    
  │ release    │ "6.12.0-164.el10…"       │ KernelRelease + KernelVersion (typed,             │ os_identity.rs │
  │            │ (string)                 │ FromStr-parsed)                                   │                │    
  ├────────────┼──────────────────────────┼───────────────────────────────────────────────────┼────────────────┤
  │ version    │ kernel build string      │ folded into KernelVersion                         │ os_identity.rs │    
  ├────────────┼──────────────────────────┼───────────────────────────────────────────────────┼────────────────┤
  │ machine    │ "x86_64" / "aarch64"     │ CpuArch enum                                      │ os_identity.rs │
  │            │ (string)                 │                                                   │                │    
  ├────────────┼──────────────────────────┼───────────────────────────────────────────────────┼────────────────┤
  │ nodename   │ hostname (unverified)    │ (intentionally not included — not an identity     │ —              │    
  │            │                          │ claim)                                            │                │    
  ├────────────┼──────────────────────────┼───────────────────────────────────────────────────┼────────────────┤
  │ domainname │                          │ (intentionally not included)                      │ —              │    
  └────────────┴──────────────────────────┴───────────────────────────────────────────────────┴────────────────┘    
  
  The trust ladder — what UMRS layers on top of those strings                                                       
                                                            
  T0  Untrusted          ← default start                                                                            
  T1  KernelAnchored     ← procfs verified via PROC_SUPER_MAGIC + PID coherence                                     
  T2  EnvAnchored        ← mountinfo ↔ statfs cross-check                                                           
  T3  SubstrateAnchored  ← package DB parsed; identity from ≥2 facts                                                
  T4  IntegrityAnchored  ← os-release bytes match packaged digest                                                   
                                                                                                                    
  TrustLevel is Ord and monotonically non-increasing — ConfidenceModel::downgrade is the only way it moves. You can 
  ask confidence.level() >= T3 before relying on substrate identity.                                                
                                                                                                                    
  The provenance trail — what every read produces                                                                   
  
  EvidenceBundle = ordered sequence of EvidenceRecord. Each record captures:                                        
  - SourceKind — Procfs / Sysfs / RegularFile / PackageDb / SymlinkTarget / StatfsResult
  - FileStat — grouped metadata from one statx(2) call (dev, ino, mode, uid/gid, nlink, size, mtime)                
  - Parse-success flag, resolved symlink target if any, observed filesystem magic                   
                                                                                                                    
  This is the audit trail uname(2) can't give you. An auditor reading EvidenceBundle can reconstruct "here's which  
  inode we read, here's the FS magic we saw, here's whether it parsed."                                             
                                                                                                                    
  The one-call entry point                                                                                          
                                                            
  use umrs_platform::detect::OsDetector;                                                                            
                                                            
  let result = OsDetector::default().detect()?;                                                                     
  // result.substrate_identity   — Option<SubstrateIdentity>
  // result.kernel_release       — Option<KernelRelease>                                                            
  // result.confidence           — ConfidenceModel (T0..T4 + contradictions)                                        
  // result.evidence             — EvidenceBundle (full provenance trail)                                           
  // result.phase_durations      — per-phase timing for AU-8                                                        
                                                                                                                    
  That's the replacement API a tool author reaches for instead of uname. It does everything uname does and returns  
  the trust grade + the evidence.                           
                                                                                                                    
  What umrs-uname the binary actually does with this        

  It's a TUI front-end over OsDetector::default().detect(). No new identity logic lives in the binary — it formats  
  the DetectionResult for the audit card. The same pattern any tool would use.
                                                                                                                    
  Underlying primitives (the layer below OsDetector)                                                                
  
  - kattrs::SecureReader<ProcfsText> / <SysfsText> — the typed wrapper that enforces path prefix at compile time +  
  verifies FS magic with fstatfs before touching bytes. This is the "L2" primitive in the tiering we sketched
  earlier.                                                                                                          
  - kattrs::tpi — Two-Path Inquiry helper (dual-read with cross-check).
  - detect::substrate::rpm_db::RpmDb — typed access to the RPM BDB/SQLite package DB.                               
                                                                                                                    
  OsDetector composes these into the seven-phase pipeline.  




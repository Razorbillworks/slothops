export interface AuditItem {
  name: string;
  zone: string;
  costEstimate: string;
  recommendation: string;
}

export function printSlothBanner(): void {
  console.log("🦥 SlothOps — GCP Idle Resource Summary");
  console.log("========================================");
}

export function formatTerminalSummary(items: AuditItem[]): void {
  printSlothBanner();
  for (const item of items) {
    console.log(`• [${item.zone}] ${item.name} -> ${item.costEstimate}`);
    console.log(`  Action: ${item.recommendation}`);
  }
  console.log("========================================\n");
}
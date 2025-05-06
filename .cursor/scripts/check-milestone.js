const fs = require('fs');
const path = require('path');

const milestonePath = path.join(__dirname, '../milestone.md');
const filesToCheck = [
  '../PRD.md',
  '../system-analysis.md',
  '../data-structure.md',
  '../api-spec.md',
  '../UIUX-Guide.md'
];

function checkMilestoneSync() {
  const milestone = fs.readFileSync(milestonePath, 'utf-8');
  let missing = [];
  filesToCheck.forEach(f => {
    if (!fs.existsSync(path.join(__dirname, f))) {
      missing.push(f);
    }
  });
  if (missing.length) {
    console.log('⚠️ 缺少以下文件：', missing.join(', '));
  }
  // 檢查 milestone 條目格式
  const requiredSections = [
    '目標說明', '對應 PRD 功能', '依賴元件/模組', '主要開發內容', '重構/最佳化建議', '預期交付物', '驗收標準', '負責人', '預計起訖日'
  ];
  const incomplete = [];
  milestone.split('### ').slice(1).forEach(block => {
    requiredSections.forEach(section => {
      if (!block.includes(section)) {
        incomplete.push({milestone: block.split('\n')[0], section});
      }
    });
  });
  if (incomplete.length) {
    console.log('⚠️ 以下 milestone 條目缺少必要欄位：');
    incomplete.forEach(i => console.log(`- ${i.milestone} 缺少 ${i.section}`));
  } else {
    console.log('✅ milestone.md 結構完整');
  }
}

checkMilestoneSync(); 
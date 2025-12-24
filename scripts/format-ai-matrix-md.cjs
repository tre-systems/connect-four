#!/usr/bin/env node
/* eslint-disable */

const fs = require('fs');
const path = require('path');
const readline = require('readline');

const OUTPUT_FILE = '../../docs/AI-MATRIX-RESULTS.md';
const OUTPUT_DIR = path.dirname(OUTPUT_FILE);

function ensureDirSync(dir) {
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
}

function parseSections(lines) {
  const sections = {
    matrix: [],
    perf: [],
    speed: [],
    recommendations: [],
    meta: [],
    date: new Date().toLocaleString('en-GB', {
      timeZone: 'Europe/London',
      hour12: false,
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    }),
  };

  let section = null;
  let inMatrix = false;
  let inPerf = false;
  let inSpeed = false;
  let inRecs = false;

  for (let i = 0; i < lines.length; ++i) {
    const line = lines[i];

    // Skip decorative lines and cargo test output
    if (
      line.startsWith('=') ||
      line.startsWith('-') ||
      line.trim() === '' ||
      line.includes('test result:') ||
      line.includes('running') ||
      line.includes('test test_') ||
      line.includes('🎉 AI Matrix test completed')
    )
      continue;

    // Section markers
    if (line.includes('Test Configuration:')) {
      section = 'meta';
      sections.meta.push(line);
      continue;
    }

    if (line.includes('MATRIX TABLE (Win Rate % of Row vs Column):')) {
      inMatrix = true;
      continue;
    }

    if (line.includes('PERFORMANCE SUMMARY:')) {
      inMatrix = false;
      inPerf = true;
      continue;
    }

    if (line.includes('SPEED ANALYSIS:')) {
      inPerf = false;
      inSpeed = true;
      continue;
    }

    if (line.includes('RECOMMENDATIONS:')) {
      inSpeed = false;
      inRecs = true;
      continue;
    }

    // Skip emoji headers
    if (
      line.startsWith('📊') ||
      line.startsWith('🏆') ||
      line.startsWith('⚡') ||
      line.startsWith('💡')
    )
      continue;

    // Collect content based on current section
    if (inMatrix && line.trim()) {
      sections.matrix.push(line);
    } else if (inPerf && line.trim()) {
      sections.perf.push(line);
    } else if (inSpeed && line.trim()) {
      sections.speed.push(line);
    } else if (inRecs && line.trim()) {
      sections.recommendations.push(line);
    } else if (section === 'meta' && line.trim()) {
      sections.meta.push(line);
    }
  }

  return sections;
}

function formatMatrixTable(matrixLines) {
  if (!matrixLines.length) return '';

  // Find the header line (contains "AI Type")
  const headerIdx = matrixLines.findIndex(l => l.includes('AI Type'));
  if (headerIdx === -1) return matrixLines.join('\n');

  const headerLine = matrixLines[headerIdx];

  // Parse fixed-width columns more carefully
  // The format is: "AI Type         Random     Heuristic  MM-Depth1 MM-Depth2 MM-Depth3 ML-Fast    ML-V2      ML-V4      ML-Hybrid  ML-PyTorch-V5"
  const colNames = [];
  let current = '';
  let inWord = false;

  for (let i = 0; i < headerLine.length; i++) {
    const char = headerLine[i];
    if (char === ' ' || char === '\t') {
      if (inWord) {
        colNames.push(current.trim());
        current = '';
        inWord = false;
      }
    } else {
      current += char;
      inWord = true;
    }
  }
  if (current.trim()) {
    colNames.push(current.trim());
  }

  // Filter out empty columns and fix "AI Type" column
  const filteredColNames = colNames.filter(name => name.length > 0);

  // Fix the first column to be "AI Type" instead of separate "AI" and "Type"
  if (
    filteredColNames.length >= 2 &&
    filteredColNames[0] === 'AI' &&
    filteredColNames[1] === 'Type'
  ) {
    filteredColNames.splice(0, 2, 'AI Type');
  }

  let table = '| ' + filteredColNames.join(' | ') + ' |\n';
  table += '| ' + filteredColNames.map(() => '---').join(' | ') + ' |\n';

  // Process data rows using similar fixed-width parsing
  for (let i = headerIdx + 1; i < matrixLines.length; ++i) {
    const row = matrixLines[i];
    if (!row.trim()) continue;

    // Parse data row with same fixed-width logic
    const cells = [];
    current = '';
    inWord = false;

    for (let j = 0; j < row.length; j++) {
      const char = row[j];
      if (char === ' ' || char === '\t') {
        if (inWord) {
          cells.push(current.trim());
          current = '';
          inWord = false;
        }
      } else {
        current += char;
        inWord = true;
      }
    }
    if (current.trim()) {
      cells.push(current.trim());
    }

    const filteredCells = cells.filter(cell => cell.length > 0);
    if (filteredCells.length < 2) continue;

    table += '| ' + filteredCells.join(' | ') + ' |\n';
  }

  return table;
}

function formatPerfSummary(perfLines) {
  if (!perfLines.length) return '';

  // Convert numbered list to markdown
  return perfLines
    .map(line => {
      const match = line.match(/^(\d+)\.\s*(.+)/);
      if (match) {
        return `${match[1]}. ${match[2]}`;
      }
      return line;
    })
    .join('\n');
}

function formatSpeedAnalysis(speedLines) {
  if (!speedLines.length) return '';

  // Convert to markdown table
  const rows = speedLines
    .map(line => {
      const match = line.match(/^(.+?):\s*([\d\.]+)ms\/move\s*\((.+?)\)/);
      if (match) {
        return [match[1], match[2], match[3]];
      }
      return null;
    })
    .filter(Boolean);

  if (!rows.length) return speedLines.join('\n');

  let table = '| AI | ms/move | Speed |\n|---|---|---|\n';
  for (const [ai, ms, speed] of rows) {
    table += `| ${ai} | ${ms} | ${speed} |\n`;
  }

  return table;
}

function formatRecommendations(lines) {
  if (!lines.length) return '';

  // Convert bullet points to markdown and filter out cargo output
  return lines
    .filter(
      line =>
        !line.includes('test result:') && !line.includes('running') && !line.includes('test test_')
    )
    .map(line => {
      const cleanLine = line.replace(/^[•\-\*]\s*/, '');
      return `- ${cleanLine}`;
    })
    .join('\n');
}

function formatMeta(metaLines) {
  if (!metaLines.length) return '';

  return metaLines
    .map(line => {
      if (line.includes('Test Configuration:')) {
        return '**Test Configuration:**';
      }
      return line;
    })
    .join('\n');
}

async function main() {
  console.log('Starting AI matrix test formatting...');

  const lines = [];
  const rl = readline.createInterface({ input: process.stdin, crlfDelay: Infinity });
  for await (const line of rl) lines.push(line);

  console.log(`Parsing ${lines.length} lines of output...`);
  const sections = parseSections(lines);

  ensureDirSync(OUTPUT_DIR);

  const md = [
    '# AI Matrix Test Results',
    '',
    `_Last updated: ${sections.date}_`,
    '',
    '## Matrix Table',
    '',
    formatMeta(sections.meta),
    '',
    formatMatrixTable(sections.matrix),
    '',
    '## Performance Summary',
    '',
    formatPerfSummary(sections.perf),
    '',
    '## Speed Analysis',
    '',
    formatSpeedAnalysis(sections.speed),
    '',
    '## Recommendations',
    '',
    formatRecommendations(sections.recommendations),
    '',
  ].join('\n');

  fs.writeFileSync(OUTPUT_FILE, md, 'utf8');
  console.log(`AI matrix results saved to ${OUTPUT_FILE}`);
}

main();

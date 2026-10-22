import { readFileSync, writeFileSync, mkdirSync, mkdtempSync, rmSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
// Maintenance-only export: macOS sips resizes the generated source; normal builds need no Node.
if (process.platform !== 'darwin') throw new Error('Regenerate icons on macOS (sips is required). Existing exports build on every platform.');
const directory = fileURLToPath(new URL('../assets/icons/', import.meta.url));
const source = join(directory, 'pistream-1024.png');
const temporary = mkdtempSync(join(tmpdir(), 'pistream-icons-'));
try {
  mkdirSync(directory, { recursive: true });
  const images = new Map();
  for (const size of [16,24,32,48,64,128,256,512,1024]) {
    const output = join(temporary, `${size}.png`);
    execFileSync('sips', ['-z', `${size}`, `${size}`, source, '--out', output], { stdio: 'ignore' });
    images.set(size, readFileSync(output));
  }
  writeFileSync(join(directory, 'pistream-128.png'), images.get(128));
  writeFileSync(join(directory, 'pistream-512.png'), images.get(512));
  const sizes=[16,24,32,48,64,128,256]; const header=Buffer.alloc(6+16*sizes.length);
  header.writeUInt16LE(1,2); header.writeUInt16LE(sizes.length,4); let offset=header.length;
  sizes.forEach((size,i)=>{const at=6+16*i;const png=images.get(size);header[at]=size===256?0:size;header[at+1]=size===256?0:size;header.writeUInt16LE(1,at+4);header.writeUInt16LE(32,at+6);header.writeUInt32LE(png.length,at+8);header.writeUInt32LE(offset,at+12);offset+=png.length;});
  writeFileSync(join(directory,'pistream.ico'),Buffer.concat([header,...sizes.map(size=>images.get(size))]));
  const chunks=[['icp4',16],['icp5',32],['icp6',64],['ic07',128],['ic08',256],['ic09',512],['ic10',1024],['ic11',32],['ic12',64],['ic13',256],['ic14',512]].map(([type,size])=>{const png=images.get(size);const h=Buffer.alloc(8);h.write(type,0,4,'ascii');h.writeUInt32BE(png.length+8,4);return Buffer.concat([h,png]);});
  const icns=Buffer.alloc(8);icns.write('icns');icns.writeUInt32BE(8+chunks.reduce((n,x)=>n+x.length,0),4);
  writeFileSync(join(directory,'PiStreamTerminal.icns'),Buffer.concat([icns,...chunks]));
  console.log('Exported PNG, Windows ICO and macOS ICNS icons.');
} finally { rmSync(temporary,{recursive:true,force:true}); }

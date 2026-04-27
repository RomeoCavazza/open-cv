<?php
header('Content-Type: application/json; charset=utf-8');
$FILE = __DIR__ . '/messages.json';
if (!file_exists($FILE)) file_put_contents($FILE, json_encode(['last_id'=>0,'items'=>[]], JSON_UNESCAPED_UNICODE));

function read_store($f){ $h=fopen($f,'r'); flock($h,LOCK_SH); $raw=stream_get_contents($h); flock($h,LOCK_UN); fclose($h); $d=json_decode($raw,true); return is_array($d)?$d:['last_id'=>0,'items'=>[]]; }
function write_store($f,$d){ $h=fopen($f,'c+'); flock($h,LOCK_EX); ftruncate($h,0); fwrite($h,json_encode($d,JSON_UNESCAPED_UNICODE)); fflush($h); flock($h,LOCK_UN); fclose($h); }

$a = $_GET['action'] ?? 'list';

if ($a === 'post') {
  $n = trim($_GET['name'] ?? '');
  $t = trim($_GET['text'] ?? '');
  if ($n === '' || $t === '') { echo json_encode(['error'=>'missing']); exit; }
  if (mb_strlen($n) > 32 || mb_strlen($t) > 1000) { echo json_encode(['error'=>'too_long']); exit; }
  $s = read_store($FILE);
  $id = ++$s['last_id'];
  $s['items'][] = ['id'=>$id,'name'=>$n,'text'=>$t,'ts'=>date('H:i:s')];
  if (count($s['items']) > 500) $s['items'] = array_slice($s['items'], -500);
  write_store($FILE,$s);
  echo json_encode(['ok'=>true]); exit;
}

if ($a === 'list') {
  $since = (int)($_GET['since_id'] ?? 0);
  $s = read_store($FILE);
  $out = array_values(array_filter($s['items'], fn($m)=>$m['id'] > $since));
  echo json_encode(['messages'=>$out], JSON_UNESCAPED_UNICODE); exit;
}

if ($a === 'reset') {
  write_store($FILE, ['last_id'=>0,'items'=>[]]);
  echo json_encode(['ok'=>true]); exit;
}

echo json_encode(['error'=>'unknown']);

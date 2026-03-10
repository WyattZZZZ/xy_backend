import json
from datetime import datetime

file_path = '/home/zz1028/xy/xy_back_end/database/products.json'

with open(file_path, 'r', encoding='utf-8') as f:
    data = json.load(f)

now_iso = "2026-03-10T20:00:00Z"

for pid, p in data.items():
    if 'condition' not in p:
        p['condition'] = 'Used'
    if 'status' not in p:
        p['status'] = 'ACTIVE'
    if 'createdAt' not in p:
        p['createdAt'] = now_iso
    if 'updatedAt' not in p:
        p['updatedAt'] = now_iso
    if 'seller_id' in p and 'sellerId' not in p:
        p['sellerId'] = p['seller_id']
    if 'video' not in p:
        p['video'] = None

with open(file_path, 'w', encoding='utf-8') as f:
    json.dump(data, f, indent=2, ensure_ascii=False)

print("Successfully updated products.json")

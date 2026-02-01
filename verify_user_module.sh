#!/bin/bash

BASE_URL="http://127.0.0.1:8081"

echo "1. Registering user..."
REGISTER_RES=$(curl -s -X POST "$BASE_URL/auth/register" \
  -H "Content-Type: application/json" \
  -d '{"username":"dev_user", "email":"dev@example.com", "password":"password123"}')
echo "Response: $REGISTER_RES"

echo -e "\n2. Logging in..."
LOGIN_RES=$(curl -s -X POST "$BASE_URL/auth/login" \
  -H "Content-Type: application/json" \
  -d '{"email":"dev@example.com", "password":"password123"}')
TOKEN=$(echo $LOGIN_RES | grep -oP '"token":"\K[^"]+')
USER_ID=$(echo $LOGIN_RES | grep -oP '"id":"\K[^"]+')
echo "Token: ${TOKEN:0:20}..."
echo "User ID: $USER_ID"

echo -e "\n3. Testing GET /user/me..."
ME_RES=$(curl -s -H "Authorization: Bearer $TOKEN" "$BASE_URL/user/me")
echo "Response: $ME_RES"

echo -e "\n4. Testing GET /user (List)..."
LIST_RES=$(curl -s "$BASE_URL/user")
echo "Response: $LIST_RES"

echo -e "\n5. Testing GET /user/$USER_ID..."
USER_RES=$(curl -s "$BASE_URL/user/$USER_ID")
echo "Response: $USER_RES"

echo -e "\n6. Testing POST /user/$USER_ID (Update)..."
UPDATE_RES=$(curl -s -X POST "$BASE_URL/user/$USER_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name":"Updated Dev User", "location":"Tokyo, Japan"}')
echo "Response: $UPDATE_RES"

echo -e "\n7. Testing DELETE /user/$USER_ID..."
DELETE_RES=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE "$BASE_URL/user/$USER_ID" \
  -H "Authorization: Bearer $TOKEN")
echo "HTTP Status: $DELETE_RES"

echo -e "\n8. Verifying deletion..."
FINAL_RES=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/user/$USER_ID")
echo "HTTP Status (should be 404): $FINAL_RES"

DATA="CREATE account SET
	name = 'ACME Inc',
	created_at = time::now();"
curl -k -L -s --compressed POST \
	--header "Accept: application/json" \
	--header "NS: test" \
	--header "DB: test" \
	--user "root:root" \
	--data "${DATA}" \
	http://localhost:8000/sql

DATA="CREATE user SET
	name = 'Test account 2',
	created_at = time::now();"
curl -k -L -s --compressed POST \
	--header "Accept: application/json" \
	--header "NS: test2" \
	--header "DB: test2" \
	--user "root:root" \
	--data "${DATA}" \
	http://localhost:8000/sql

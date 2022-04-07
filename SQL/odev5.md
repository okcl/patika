## ödev 5

1- film tablosunda bulunan ve film ismi (title) 'n' karakteri ile biten en uzun (length) 5 filmi sıralayınız.

*Cevap*
```sql
SELECT title
FROM film
WHERE title LIKE '%n'
ORDER BY length
LIMIT 5;
```
2- film tablosunda bulunan ve film ismi (title) 'n' karakteri ile biten en kısa (length) ikinci(6,7,8,9,10) 5 filmi(6,7,8,9,10) sıralayınız.

*Cevap*
```sql
SELECT title
FROM film
WHERE title LIKE '%n'
ORDER BY length DESC
OFFSET 6;
LIMIT 5;
```
3- customer tablosunda bulunan last_name sütununa göre azalan yapılan sıralamada store_id 1 olmak koşuluyla ilk 4 veriyi sıralayınız.

*Cevap*
```sql
SELECT *
FROM customer
WHERE store_id = 1
ORDER BY last_name
LIMIT 4;
```


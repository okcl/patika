## ödev 4 

1- film tablosunda bulunan replacement_cost sütununda bulunan birbirinden farklı değerleri sıralayınız.

*Cevap*
```sql
SELECT DISTINCT replacement_cost
FROM film;
```
2- film tablosunda bulunan replacement_cost sütununda birbirinden farklı kaç tane veri vardır?

*Cevap*
```sql
SELECT DISTINCT COUNT(*)
FROM film;
```
3- film tablosunda bulunan film isimlerinde (title) kaç tanesini T karakteri ile başlar ve aynı zamanda rating 'G' ye eşittir?

*Cevap*
```sql
SELECT COUNT(title)
FROM film
WHERE title LIKE 't%'
AND rating = 'G';
```
4- country tablosunda bulunan ülke isimlerinden (country) kaç tanesi 5 karakterden oluşmaktadır?

*Cevap*
```sql
SELECT COUNT(*)
FROM country
WHERE length(country) = 5;
```
5- city tablosundaki şehir isimlerinin kaç tanesi 'R' veya r karakteri ile biter?

*Cevap*
```sql
SELECT COUNT(*)
FROM city
WHERE city LIKE '%r'
OR city LIKE '%R';
```
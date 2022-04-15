1. test veritabanınızda employee isimli sütun bilgileri id(INTEGER), name VARCHAR(50), birthday DATE, email VARCHAR(100) olan bir tablo oluşturalım.
```sql
CREATE TABLE employee(
    id INT, 
    name VARCHAR(50), 
    birthday DATE, 
    email VARCHAR(100);

    create table employee (
	id INT,
	name VARCHAR(50),
	birthday DATE,
	email VARCHAR(50)
);
```
2. Oluşturduğumuz employee tablosuna 'Mockaroo' servisini kullanarak 50 adet veri ekleyelim.

```sql
insert into employee (id, name, birthday, email) values (1, 'Hieronymus', '3/14/2004', 'hniland0@fda.gov');
insert into employee (id, name, birthday, email) values (2, 'Carver', '6/11/2001', 'crichold1@bandcamp.com');
insert into employee (id, name, birthday, email) values (3, 'Min', '12/22/1995', 'mmacmychem2@cpanel.net');
insert into employee (id, name, birthday, email) values (4, 'Lin', '7/17/2006', 'lburditt3@taobao.com');
insert into employee (id, name, birthday, email) values (5, 'Leupold', '6/6/2002', 'lspleving4@purevolume.com');
insert into employee (id, name, birthday, email) values (6, 'Aguie', '5/19/1994', 'ameindl5@arstechnica.com');
insert into employee (id, name, birthday, email) values (7, 'Hirsch', '10/11/1991', 'hmathwin6@gizmodo.com');
insert into employee (id, name, birthday, email) values (8, 'Othilie', '11/6/1990', 'oadamson7@reference.com');
insert into employee (id, name, birthday, email) values (9, 'Sansone', '8/29/1999', 'sshillaber8@fotki.com');
insert into employee (id, name, birthday, email) values (10, 'Alyssa', '8/17/1995', 'acomino9@washingtonpost.com');
insert into employee (id, name, birthday, email) values (11, 'Webb', '9/2/2004', 'wboolea@ibm.com');
insert into employee (id, name, birthday, email) values (12, 'Tuesday', '11/30/2004', 'tchomleyb@cnet.com');
insert into employee (id, name, birthday, email) values (13, 'Arvy', '1/30/1998', 'afallac@stanford.edu');
insert into employee (id, name, birthday, email) values (14, 'Corbet', '6/19/2004', 'cbraganzad@youtube.com');
insert into employee (id, name, birthday, email) values (15, 'Shalna', '7/3/1995', 'sstonehousee@xinhuanet.com');
insert into employee (id, name, birthday, email) values (16, 'Rudie', '3/10/2007', 'raslamf@livejournal.com');
insert into employee (id, name, birthday, email) values (17, 'Free', '4/22/1996', 'fdiantoniog@storify.com');
insert into employee (id, name, birthday, email) values (18, 'Shaw', '10/11/1995', 'srennerh@csmonitor.com');
insert into employee (id, name, birthday, email) values (19, 'Nicole', '5/25/2003', 'ndorsayi@vk.com');
insert into employee (id, name, birthday, email) values (20, 'Simon', '5/12/1992', 'sbramwichj@google.co.jp');
insert into employee (id, name, birthday, email) values (21, 'Foster', '5/3/1997', 'fgallaherk@mapy.cz');
insert into employee (id, name, birthday, email) values (22, 'Lissi', '8/25/2002', 'lmeehanl@networkadvertising.org');
insert into employee (id, name, birthday, email) values (23, 'Berny', '3/24/2005', 'bslinnm@jiathis.com');
insert into employee (id, name, birthday, email) values (24, 'Innis', '5/24/2005', 'ikrolln@slideshare.net');
insert into employee (id, name, birthday, email) values (25, 'Doralyn', '4/20/1996', 'dfromeo@squarespace.com');
insert into employee (id, name, birthday, email) values (26, 'Nowell', '1/28/2001', 'nbaintonp@admin.ch');
insert into employee (id, name, birthday, email) values (27, 'Barnabe', '8/29/1992', 'brawkesbyq@furl.net');
insert into employee (id, name, birthday, email) values (28, 'Marysa', '1/19/1997', 'msnarr@bluehost.com');
insert into employee (id, name, birthday, email) values (29, 'Monica', '9/10/1991', 'mgludors@prnewswire.com');
insert into employee (id, name, birthday, email) values (30, 'Tracey', '5/14/1995', 'tmarchit@amazon.co.uk');
insert into employee (id, name, birthday, email) values (31, 'Deerdre', '9/21/1993', 'dspurieru@wsj.com');
insert into employee (id, name, birthday, email) values (32, 'Archambault', '11/19/2000', 'awasielewskiv@networksolutions.com');
insert into employee (id, name, birthday, email) values (33, 'Blaine', '7/6/1998', 'bdundredgew@blogspot.com');
insert into employee (id, name, birthday, email) values (34, 'Sheena', '10/20/1996', 'splanx@google.com.au');
insert into employee (id, name, birthday, email) values (35, 'Terry', '7/7/1994', 'tdungatey@bbb.org');
insert into employee (id, name, birthday, email) values (36, 'Marya', '7/14/2002', 'mchappellz@bigcartel.com');
insert into employee (id, name, birthday, email) values (37, 'Dame', '9/22/1999', 'dvaudin10@google.it');
insert into employee (id, name, birthday, email) values (38, 'Saul', '6/24/1993', 'sfetter11@psu.edu');
insert into employee (id, name, birthday, email) values (39, 'Khalil', '4/23/2001', 'kmcilwreath12@qq.com');
insert into employee (id, name, birthday, email) values (40, 'Gale', '4/25/1993', 'grevey13@goo.gl');
insert into employee (id, name, birthday, email) values (41, 'Judah', '1/8/1998', 'jarnowitz14@friendfeed.com');
insert into employee (id, name, birthday, email) values (42, 'Caren', '9/16/1995', 'cmcgurgan15@addtoany.com');
insert into employee (id, name, birthday, email) values (43, 'Lelia', '8/6/1997', 'lfincke16@yellowpages.com');
insert into employee (id, name, birthday, email) values (44, 'Bale', '7/6/1991', 'bclere17@vkontakte.ru');
insert into employee (id, name, birthday, email) values (45, 'Cesare', '7/2/1993', 'ctrevaskus18@craigslist.org');
insert into employee (id, name, birthday, email) values (46, 'Daisi', '10/16/1999', 'dstebbings19@ocn.ne.jp');
insert into employee (id, name, birthday, email) values (47, 'Orland', '7/20/2006', 'ohedan1a@globo.com');
insert into employee (id, name, birthday, email) values (48, 'Celie', '4/22/2004', 'csorrell1b@nifty.com');
insert into employee (id, name, birthday, email) values (49, 'Audie', '4/1/2005', 'ayannoni1c@com.com');
insert into employee (id, name, birthday, email) values (50, 'Kiel', '3/5/2002', 'kciotto1d@fastcompany.com');
```
3. Sütunların her birine göre diğer sütunları güncelleyecek 5 adet UPDATE işlemi yapalım.

```sql
UPDATE employee 
name='Ahmet',
birthday = '2000-06-10',
email = 'ahmet123@gmail.com'
WHERE id=1;
```

```sql
UPDATE employee
name='Selin',
birthday = '1998-05-08',
email = 'selin321@gmail.com'
WHERE id=2
```

```sql
UPDATE employee
name='Melis',
birthday = '2001-11-28',
email = 'mel1s@gmail.com'
WHERE id=3
```

```sql
UPDATE employee
name='Hazal',
birthday = '1998-10-01',
email = 'h4zal@gmail.com'
WHERE id=4
```


```sql
UPDATE employee
name='Mehmet',
birthday = '1995-01-*1',
email = 'Mehmet2@gmail.com'
WHERE id=5
```
4. Sütunların her birine göre ilgili satırı silecek 5 adet DELETE işlemi yapalım.


```sql
DELETE employee 
WHERE id=1
```
```sql
DELETE employee 
WHERE id=2
```
```sql
DELETE employee 
WHERE id=3
```
```sql
DELETE employee 
WHERE id=4
```
```sql
DELETE employee 
WHERE id=5
```


-- "secret33" is the password for all the users

INSERT INTO users (id, username, email, password, avatar)
VALUES (1, 'Julie', 'julie@gmail.com', 'bbb7e34422270b07edfe2f4ce3087c1712567fc2c108487b295274a15d922cdc', 'https://i1.sndcdn.com/artworks-000102605413-57gt1d-t500x500.jpg');

INSERT INTO users (id, username, email, password, avatar)
VALUES (2, 'Ashley', 'ashley@gmail.com', 'bbb7e34422270b07edfe2f4ce3087c1712567fc2c108487b295274a15d922cdc', 'https://randomuser.me/api/portraits/women/20.jpg');

INSERT INTO users (id, username, email, password, avatar)
VALUES (3, 'Anna', 'anna@gmail.com', 'bbb7e34422270b07edfe2f4ce3087c1712567fc2c108487b295274a15d922cdc', 'https://randomuser.me/api/portraits/women/60.jpg');

INSERT INTO users (id, username, email, password, avatar)
VALUES (4, 'Jill', 'jill@gmail.com', 'bbb7e34422270b07edfe2f4ce3087c1712567fc2c108487b295274a15d922cdc', 'https://s3.ir-thr-at1.arvanstorage.com/messenger/user-36-avatar.jpeg');

INSERT INTO users (id, username, email, password, avatar)
VALUES (5, 'Lena', 'lena@gmail.com', 'bbb7e34422270b07edfe2f4ce3087c1712567fc2c108487b295274a15d922cdc', 'https://s3.ir-thr-at1.arvanstorage.com/messenger/user-40-avatar.jpeg');

INSERT INTO users (id, username, email, password, avatar)
VALUES (6, 'Oliver', 'oliver@gmail.com', 'bbb7e34422270b07edfe2f4ce3087c1712567fc2c108487b295274a15d922cdc', 'https://s3.ir-thr-at1.arvanstorage.com/messenger/user-39-avatar.jpeg');

INSERT INTO users (id, username, email, password, avatar)
VALUES (7, 'Kevin', 'kevin@gmail.com', 'bbb7e34422270b07edfe2f4ce3087c1712567fc2c108487b295274a15d922cdc', 'https://s3.ir-thr-at1.arvanstorage.com/messenger/user-39-avatar.jpeg');

INSERT INTO users (id, username, email, password, avatar)
VALUES (8, 'Mason', 'mason@gmail.com', 'bbb7e34422270b07edfe2f4ce3087c1712567fc2c108487b295274a15d922cdc', 'https://s3.ir-thr-at1.arvanstorage.com/messenger/user-39-avatar.jpeg');

INSERT INTO users (id, username, email, password, avatar)
VALUES (9, 'Sylvanas', 'sylvanas@gmail.com', 'bbb7e34422270b07edfe2f4ce3087c1712567fc2c108487b295274a15d922cdc', 'https://images-wixmp-ed30a86b8c4ca887773594c2.wixmp.com/f/241f2636-882f-4357-9e99-b9a3cbcfa999/dd4w16p-b25405ea-a1af-4907-adeb-a37c4a238c6e.jpg/v1/fill/w_1024,h_1387,q_75,strp/sylvanas_by_z__ed_dd4w16p-fullview.jpg?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1cm46YXBwOjdlMGQxODg5ODIyNjQzNzNhNWYwZDQxNWVhMGQyNmUwIiwiaXNzIjoidXJuOmFwcDo3ZTBkMTg4OTgyMjY0MzczYTVmMGQ0MTVlYTBkMjZlMCIsIm9iaiI6W1t7ImhlaWdodCI6Ijw9MTM4NyIsInBhdGgiOiJcL2ZcLzI0MWYyNjM2LTg4MmYtNDM1Ny05ZTk5LWI5YTNjYmNmYTk5OVwvZGQ0dzE2cC1iMjU0MDVlYS1hMWFmLTQ5MDctYWRlYi1hMzdjNGEyMzhjNmUuanBnIiwid2lkdGgiOiI8PTEwMjQifV1dLCJhdWQiOlsidXJuOnNlcnZpY2U6aW1hZ2Uub3BlcmF0aW9ucyJdfQ.4Hen26VHz-90JkqBON_5uzzIwkUgyU14kTcdlqZwfE0');


-- create also profile relation for users

INSERT INTO profiles (id, user_id, status, description)
VALUES (1, 1, 'Do Everything in Love', 'I am an experienced joiner with well developed skills and experience in groundwork, concrete finishing and steel fixing and have worked in the construction industry since 1982. I am also a skilled labourer who has supported many different trades over the years. I have a full clean UK driving licence with entitlement of up to 7.5 tonne. I am keen to return to work after a period of training and personal development which has broadened my skills and experiences.');

INSERT INTO profiles (id, user_id, status, description)
VALUES (2, 2, 'If you can dream it, you can do it', 'I am a professionally qualified fire engineer with 7 years experience. I have recently achieved RTITB accreditation in the use of Counterbalance fork lift trucks and I am seeking employment that will make best use of my skills and allow me to develop them further. I am determined and enthusiastic, I have developed good planning & organisational skills and am confident working independently or as part of a team. I am flexible regarding working hours and am able to work a range of shifts.');

INSERT INTO profiles (id, user_id, status, description)
VALUES (3, 3, 'Don`t tell people your dreams, show them', 'I am a talented, ambitious and hardworking individual, with broad skills and experience in digital and printed marketing, social media and leading projects.
Furthermore, I am adept at handling multiple tasks on a daily basis competently and at working well under pressure.

A key strength is communication; building strong relationships with people in order to deliver the best results.

Recently, I completed an Open degree, including Business and Design modules at the Open University and I am now fully employed by Clearly Presented as a Digital Media Manager.');

INSERT INTO profiles (id, user_id, status, description)
VALUES (4, 4, 'Life is a beautfiul struggle', 'Exceptionally skilled Journalism graduate with a knack for finding stories and presenting them to the public. Looking for an entry-level position in a media house that stands true to the ethics of journalism and would utilize my creative and professional skills to best use. Possess good communication skills and have an eye for detail. Flexible and willing to work in any environment as and when needed.');

INSERT INTO profiles (id, user_id, status, description)
VALUES (5, 5, 'The worst kind of sad is not being able to explain why', 'A highly innovative individual with a keen interest in developing creative case strategies and writing effective briefs. Possess excellent argument techniques and ideas that help in winning cases. Ready to work in a dynamic environment that offers opportunities to grow and learn new things in the legal field.

A profile summary is a synopsis of your skills and expertise. And since you are just starting your career, it is always a great idea to put forth your skills, goals, and experience to take over on the dream job you are looking for.');

INSERT INTO profiles (id, user_id, status, description)
VALUES (6, 6, 'Everyday is a second chance', 'A creative and strategic thinker motivated to build a career in Public Relations. Capability to communicate and generate brand awareness in an innovative way. Strong interpersonal communication. Skillful at event planning and organizing. Willing to explore PR strategies that help business increase revenue.');

INSERT INTO profiles (id, user_id, status, description)
VALUES (7, 7, 'Happiness is an inside job', 'Seeking an opportunity to serve as a School Teacher for a reputed group. B.ed and Masters in ABC. Skilled in Classroom Management and Lesson Planning. Capable of building an open and interactive environment to help students express themselves in a better way. Proficient in a range of teaching styles and communication.');

INSERT INTO profiles (id, user_id, status, description)
VALUES (8, 8, 'Who hurt me? “My own expectations.”', 'I am a dedicated, organized and methodical individual. I have good interpersonal skills, am an excellent team worker and am keen and very willing to learn and develop new skills. I am reliable and dependable and often seek new responsibilities within a wide range of employment areas. I have an active and dynamic approach to work and getting things done. I am determined and decisive. I identify and develop opportunities.');

INSERT INTO profiles (id, user_id, status, description)
VALUES (9, 9, 'You only fail when you stop trying', 'I am a hardworking and ambitious individual with a great passion for the transport and logistics industry. I am currently in my second year of studying BA Logistics and Supply Chain Management at Aston University. I have excellent communication skills, enabling me to effectively communicate with a wide range of people. I am seeing a part-time position in the industry in which I can put into practice my knowledge and experience, ultimately benefiting the operations of the organisation that I work for.
The above personal statement is clear and informative, making it clear that the applicant is a student, currently completing their university degree, and are looking to work part-time in the industry.
It is always a good idea, as the candidate has done in this example, to clearly mention your availability for work and also the reasons for why you are seeking work. “Making money” is not a good enough reason for an employer to give you a job. The candidate has mentioned that they wish to put into practice what they have learned and make a positive contribution to the employer.');

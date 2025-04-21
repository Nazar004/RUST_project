CREATE TABLE Transactions (
	tran_id 	  SERIAL NOT NULL,
    tran_type    VARCHAR NOT NULL,
    user_id      INT NOT NULL,
    tran_source  VARCHAR(40) NOT NULL,
    date         Varchar(12) NOT NULL,
    tran_amount  NUMERIC NOT NULL,
    tran_comment VARCHAR(120),
    PRIMARY KEY (tran_type, user_id,tran_id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

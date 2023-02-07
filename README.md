# vprytz-hash

Database management system.

## Data structure

The database only supports one table. All data is stored in `data.csv` which is essentially just read into memory when the program starts. The `data.csv` file looks something like this.

```csv
id,first_name,last_name,email
0,Vilhelm,Prytz,vilhelm@prytznet.se
```

The first line of the CSV file contains all keys available in the database. All the following lines contain the data of the database, i.e. the records.

## Author

Created by Vilhelm Prytz <vilhelm@prytznet.se> / <vprytz@kth.se>.

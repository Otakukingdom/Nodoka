//
// Created by mistlight on 1/1/2017.
//

#include <QtSql/QSqlDatabase>
#include <QtSql/QSqlQuery>
#include "DatabaseConnect.h"

bool ::Core::openDb() {
    QSqlDatabase db = QSqlDatabase::addDatabase("QSQLITE");
    db.setDatabaseName("nodoka.db");

    if(!db.open()) {
        return false;
    }

    QSqlQuery query;
    query.exec("CREATE TABLE IF NOT EXISTS metadata ("
                       "key text PRIMARY KEY,"
                       "value text"
                       ")");

    query.exec("CREATE TABLE IF NOT EXISTS directories("
                       "full_path text PRIMARY KEY,"
                       "created_at text,"
                       "last_scanned text"
                       ")");

    return true;
}

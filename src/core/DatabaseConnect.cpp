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

    query.exec("CREATE TABLE IF NOT EXISTS audiobooks("
                       "id INTEGER AUTOINCREMENT,"
                       "directory TEXT,"
                       "name TEXT,"
                       "full_path TEXT,"
                       "completeness INTEGER,"
                       "default_order INTEGER,"
                       "selected_file INTEGER,"
                       "created_at TEXT"
                       ")");

    query.exec("CREATE TABLE IF NOT EXISTS audiobook_file("
                       "id INTEGER AUTOINCREMENT,"
                       "audiobook_id INTEGER,"
                       "name TEXT,"
                       "full_path TEXT,"
                       "length_of_file TEXT,"
                       "position INTEGER,"
                       "completeness INTEGER,"
                       "seek_position REAL,"
                       "file_exists BOOL,"
                       "created_at TEXT"
                       ")");

    return true;
}

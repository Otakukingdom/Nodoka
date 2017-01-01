//
// Created by mistlight on 1/1/2017.
//

#include <QtSql/QSqlRecord>
#include <ctime>
#include <sstream>
#include "Directory.h"

Directory::Directory(QObject *parent) : QSqlTableModel(parent) {
    this->setTable("directories");
}

void Directory::addDirectory(QString path) {
    QSqlRecord record;

    // create the time string
    std::time_t now = std::time(0);
    std::string timeString;
    std::stringstream s(timeString);
    s << now;

    record.setValue("full_path", path);
    record.setValue("created_at", QString::fromStdString(timeString));
}

void Directory::removeDirectory(QString path) {

}


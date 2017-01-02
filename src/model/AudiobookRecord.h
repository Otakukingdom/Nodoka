//
// Created by mistlight on 1/2/2017.
//

#ifndef NODOKANATIVE_AUDIOBOOKRECORD_H
#define NODOKANATIVE_AUDIOBOOKRECORD_H

#include <QtSql>

class AudiobookRecord : public QSqlRecord {
    void setup();
    void setValues();
    QString path;
    QString calculateName();

public:
    AudiobookRecord(QString path);

};


#endif //NODOKANATIVE_AUDIOBOOKRECORD_H

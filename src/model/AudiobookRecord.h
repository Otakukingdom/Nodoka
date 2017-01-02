//
// Created by mistlight on 1/2/2017.
//

#ifndef NODOKANATIVE_AUDIOBOOKRECORD_H
#define NODOKANATIVE_AUDIOBOOKRECORD_H

#include <QtSql>

class AudiobookRecord : public QSqlRecord {
private:
    bool readMode;

    void setup();
    void setValues();
    QString path;
    QString calculateName();

public:
    AudiobookRecord();
    AudiobookRecord(QString path, bool readMode);

};


#endif //NODOKANATIVE_AUDIOBOOKRECORD_H

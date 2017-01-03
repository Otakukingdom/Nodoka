//
// Created by mistlight on 1/2/2017.
//

#ifndef NODOKANATIVE_AUDIOBOOKFILERECORD_H
#define NODOKANATIVE_AUDIOBOOKFILERECORD_H

#include <QtSql>

class AudiobookFileRecord : public QSqlRecord {

private:
    bool readMode;

    void setup();
    void setValues();

public:
    AudiobookFileRecord();
    AudiobookFileRecord(QString path, bool readMode);

    void setInitValues();
};


#endif //NODOKANATIVE_AUDIOBOOKFILERECORD_H

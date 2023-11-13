## Some keypoints I didn't mentioned...

### Send and Sync trait

This two traits are used to mark the capability of safety across thread

Types with Send implemented is safe to be moved to another thread, for example, Arc and Mutex


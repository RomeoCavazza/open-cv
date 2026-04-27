# platforms/schemas.py
from pydantic import BaseModel, Field
from typing import Optional
from datetime import datetime

class PlatformBase(BaseModel):
    name: str
    api_key: Optional[str] = None

class PlatformCreate(PlatformBase):
    pass

class PlatformUpdate(BaseModel):
    name: Optional[str] = None
    api_key: Optional[str] = None

class PlatformResponse(PlatformBase):
    id: int
    created_at: Optional[datetime] = None
    
    class Config:
        from_attributes = True
